-- Create guild_app schema if it doesn't exist
CREATE SCHEMA IF NOT EXISTS guild_app;

-- Set search path to our schema
SET search_path TO guild_app;

-- Create UUIDs extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create enum types
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'quest_status') THEN
        CREATE TYPE guild_app.quest_status AS ENUM ('AVAILABLE', 'IN_PROGRESS', 'COMPLETED', 'FAILED');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'character_type') THEN
        CREATE TYPE guild_app.character_type AS ENUM ('PLAYER', 'NPC', 'MONSTER', 'AI_TEAMMATE');
    END IF;
END$$;

-- Create tables

-- Skills table
CREATE TABLE IF NOT EXISTS guild_app.skills (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    category VARCHAR(50) NOT NULL,
    level INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Guilds table
CREATE TABLE IF NOT EXISTS guild_app.guilds (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    guild_master_id UUID,
    town_id UUID,
    reward_divider_percentage INTEGER NOT NULL DEFAULT 80, -- % to distribute to members
    stash_reward INTEGER NOT NULL DEFAULT 0, -- Accumulated rewards for guild
    quests_completed INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- AI Teammates table
CREATE TABLE IF NOT EXISTS guild_app.ai_teammates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    guild_id UUID NOT NULL REFERENCES guild_app.guilds(id) ON DELETE CASCADE,
    character_id UUID UNIQUE, -- Will be set after character is created
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Towns table
CREATE TABLE IF NOT EXISTS guild_app.towns (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Quests table
CREATE TABLE IF NOT EXISTS guild_app.quests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(200) NOT NULL,
    description TEXT,
    difficulty INTEGER NOT NULL DEFAULT 1,
    reward TEXT,
    xp_reward INTEGER DEFAULT 0,
    gold_reward INTEGER DEFAULT 0,
    status quest_status DEFAULT 'AVAILABLE',
    assigned_guild_id UUID REFERENCES guild_app.guilds(id) ON DELETE SET NULL,
    assigned_player_id UUID, -- Will reference characters.id
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Teams table
CREATE TABLE IF NOT EXISTS guild_app.teams (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Characters table
CREATE TABLE IF NOT EXISTS guild_app.characters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    type character_type NOT NULL DEFAULT 'PLAYER',
    level INTEGER NOT NULL DEFAULT 1,
    xp INTEGER NOT NULL DEFAULT 0,
    active_guild_id UUID REFERENCES guild_app.guilds(id) ON DELETE SET NULL,
    team_id UUID REFERENCES guild_app.teams(id) ON DELETE SET NULL,
    properties JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT check_ai_teammate_type CHECK (
        (type = 'AI_TEAMMATE' AND EXISTS (
            SELECT 1 FROM guild_app.ai_teammates 
            WHERE character_id = characters.id
        )) OR
        type != 'AI_TEAMMATE'
    )
);

-- Update the foreign key reference in ai_teammates
ALTER TABLE guild_app.ai_teammates
ADD CONSTRAINT fk_ai_teammates_character
FOREIGN KEY (character_id) REFERENCES guild_app.characters(id) ON DELETE SET NULL;

-- Update quests foreign key reference
ALTER TABLE guild_app.quests
ADD CONSTRAINT fk_quests_assigned_player
FOREIGN KEY (assigned_player_id) REFERENCES guild_app.characters(id) ON DELETE SET NULL;

-- Update guilds foreign key references
ALTER TABLE guild_app.guilds
ADD CONSTRAINT fk_guilds_guild_master
FOREIGN KEY (guild_master_id) REFERENCES guild_app.characters(id) ON DELETE SET NULL;

ALTER TABLE guild_app.guilds
ADD CONSTRAINT fk_guilds_town
FOREIGN KEY (town_id) REFERENCES guild_app.towns(id) ON DELETE SET NULL;

-- Character Guild membership (many-to-many, up to 3 guilds per character)
CREATE TABLE IF NOT EXISTS guild_app.character_guilds (
    character_id UUID REFERENCES guild_app.characters(id) ON DELETE CASCADE,
    guild_id UUID REFERENCES guild_app.guilds(id) ON DELETE CASCADE,
    is_active BOOLEAN DEFAULT FALSE,
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    PRIMARY KEY (character_id, guild_id),
    CONSTRAINT max_three_guilds CHECK (
        (SELECT COUNT(*) FROM guild_app.character_guilds 
         WHERE character_id = character_guilds.character_id) <= 3
    ),
    CONSTRAINT one_active_guild CHECK (
        NOT is_active OR NOT EXISTS (
            SELECT 1 FROM guild_app.character_guilds 
            WHERE character_id = character_guilds.character_id 
            AND guild_id != character_guilds.guild_id 
            AND is_active = TRUE
        )
    )
);

-- Quest required skills table (Many-to-many relationship)
CREATE TABLE IF NOT EXISTS guild_app.quest_required_skills (
    quest_id UUID REFERENCES guild_app.quests(id) ON DELETE CASCADE,
    skill_id UUID REFERENCES guild_app.skills(id) ON DELETE CASCADE,
    minimum_level INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (quest_id, skill_id)
);

-- Character skills table (Many-to-many relationship)
CREATE TABLE IF NOT EXISTS guild_app.character_skills (
    character_id UUID REFERENCES guild_app.characters(id) ON DELETE CASCADE,
    skill_id UUID REFERENCES guild_app.skills(id) ON DELETE CASCADE,
    level INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (character_id, skill_id)
);

-- Completed quests tracking
CREATE TABLE IF NOT EXISTS guild_app.completed_quests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    quest_id UUID REFERENCES guild_app.quests(id) ON DELETE CASCADE,
    character_id UUID REFERENCES guild_app.characters(id) ON DELETE CASCADE,
    guild_id UUID REFERENCES guild_app.guilds(id) ON DELETE CASCADE,
    completion_date TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    reward_amount INTEGER DEFAULT 0,
    is_guild_credited BOOLEAN DEFAULT FALSE
);

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION guild_app.update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create triggers to update timestamps
CREATE TRIGGER update_skills_updated_at BEFORE UPDATE ON guild_app.skills
    FOR EACH ROW EXECUTE FUNCTION guild_app.update_updated_at_column();

CREATE TRIGGER update_quests_updated_at BEFORE UPDATE ON guild_app.quests
    FOR EACH ROW EXECUTE FUNCTION guild_app.update_updated_at_column();

CREATE TRIGGER update_teams_updated_at BEFORE UPDATE ON guild_app.teams
    FOR EACH ROW EXECUTE FUNCTION guild_app.update_updated_at_column();

CREATE TRIGGER update_characters_updated_at BEFORE UPDATE ON guild_app.characters
    FOR EACH ROW EXECUTE FUNCTION guild_app.update_updated_at_column();
    
CREATE TRIGGER update_guilds_updated_at BEFORE UPDATE ON guild_app.guilds
    FOR EACH ROW EXECUTE FUNCTION guild_app.update_updated_at_column();
    
CREATE TRIGGER update_towns_updated_at BEFORE UPDATE ON guild_app.towns
    FOR EACH ROW EXECUTE FUNCTION guild_app.update_updated_at_column();
    
CREATE TRIGGER update_ai_teammates_updated_at BEFORE UPDATE ON guild_app.ai_teammates
    FOR EACH ROW EXECUTE FUNCTION guild_app.update_updated_at_column();

-- Function to ensure a character can only be active in one guild
CREATE OR REPLACE FUNCTION guild_app.update_active_guild()
RETURNS TRIGGER AS $$
BEGIN
    -- If we're setting a character as active in a guild
    IF NEW.is_active = TRUE THEN
        -- Update the character's active_guild_id
        UPDATE guild_app.characters
        SET active_guild_id = NEW.guild_id
        WHERE id = NEW.character_id;
        
        -- Set all other guild memberships to inactive
        UPDATE guild_app.character_guilds
        SET is_active = FALSE
        WHERE character_id = NEW.character_id
          AND guild_id != NEW.guild_id;
    ELSIF OLD.is_active = TRUE AND NEW.is_active = FALSE THEN
        -- If deactivating, clear the active guild
        UPDATE guild_app.characters
        SET active_guild_id = NULL
        WHERE id = NEW.character_id;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_active_guild_trigger
AFTER INSERT OR UPDATE ON guild_app.character_guilds
FOR EACH ROW EXECUTE FUNCTION guild_app.update_active_guild();

-- Function to ensure guild master is always a member of the guild
CREATE OR REPLACE FUNCTION guild_app.ensure_guild_master_membership()
RETURNS TRIGGER AS $$
BEGIN
    -- When guild is created or guild master is updated
    IF NEW.guild_master_id IS NOT NULL THEN
        -- Ensure guild master is a member of the guild
        INSERT INTO guild_app.character_guilds (character_id, guild_id, is_active)
        VALUES (NEW.guild_master_id, NEW.id, TRUE)
        ON CONFLICT (character_id, guild_id) DO UPDATE
        SET is_active = TRUE;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER ensure_guild_master_membership_trigger
AFTER INSERT OR UPDATE OF guild_master_id ON guild_app.guilds
FOR EACH ROW EXECUTE FUNCTION guild_app.ensure_guild_master_membership();

-- Function to create AI teammate when a guild is created
CREATE OR REPLACE FUNCTION guild_app.create_ai_teammate_for_guild()
RETURNS TRIGGER AS $$
DECLARE
    new_character_id UUID;
BEGIN
    -- Insert a new AI teammate character
    INSERT INTO guild_app.characters (
        name, 
        type, 
        level, 
        xp, 
        active_guild_id,
        properties
    )
    VALUES (
        'AI Teammate - ' || NEW.name, 
        'AI_TEAMMATE', 
        1, 
        0, 
        NEW.id,
        jsonb_build_object('is_ai_teammate', true, 'guild_id', NEW.id)
    )
    RETURNING id INTO new_character_id;
    
    -- Create AI teammate record
    INSERT INTO guild_app.ai_teammates (
        guild_id,
        character_id
    )
    VALUES (
        NEW.id,
        new_character_id
    );
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER create_ai_teammate_trigger
AFTER INSERT ON guild_app.guilds
FOR EACH ROW EXECUTE FUNCTION guild_app.create_ai_teammate_for_guild();

-- Function to update guild quests_completed count
CREATE OR REPLACE FUNCTION guild_app.update_guild_quests_completed()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.is_guild_credited = TRUE AND (OLD IS NULL OR OLD.is_guild_credited = FALSE) THEN
        UPDATE guild_app.guilds
        SET quests_completed = quests_completed + 1
        WHERE id = NEW.guild_id;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_guild_quests_completed_trigger
AFTER INSERT OR UPDATE OF is_guild_credited ON guild_app.completed_quests
FOR EACH ROW EXECUTE FUNCTION guild_app.update_guild_quests_completed();

-- Functions for querying quests by skills

-- Function to find quests that require exactly the provided skills
CREATE OR REPLACE FUNCTION guild_app.find_quests_by_exact_skills(skill_ids UUID[])
RETURNS TABLE (
    quest_id UUID,
    title VARCHAR,
    description TEXT,
    difficulty INTEGER
) AS $$
BEGIN
    RETURN QUERY
    WITH quest_skill_counts AS (
        SELECT 
            q.id AS quest_id,
            q.title,
            q.description,
            q.difficulty,
            COUNT(qrs.skill_id) AS skill_count,
            (SELECT COUNT(*) FROM guild_app.quest_required_skills WHERE quest_id = q.id) AS total_skills_required
        FROM 
            guild_app.quests q
        JOIN 
            guild_app.quest_required_skills qrs ON q.id = qrs.quest_id
        WHERE 
            qrs.skill_id = ANY(skill_ids)
            AND q.status = 'AVAILABLE'
        GROUP BY 
            q.id
    )
    SELECT 
        qsc.quest_id,
        qsc.title,
        qsc.description,
        qsc.difficulty
    FROM 
        quest_skill_counts qsc
    WHERE 
        qsc.skill_count = qsc.total_skills_required
        AND qsc.skill_count = array_length(skill_ids, 1)
    ORDER BY
        qsc.difficulty;
END;
$$ LANGUAGE plpgsql;

-- Function to find guilds by skills
CREATE OR REPLACE FUNCTION guild_app.find_guilds_by_skills(skill_ids UUID[])
RETURNS TABLE (
    guild_id UUID,
    guild_name VARCHAR,
    description TEXT,
    matching_skills_count INTEGER,
    total_skills INTEGER,
    quests_completed INTEGER
) AS $$
BEGIN
    RETURN QUERY
    WITH guild_skills AS (
        SELECT 
            g.id AS guild_id,
            g.name AS guild_name,
            g.description,
            g.quests_completed,
            COUNT(DISTINCT cs.skill_id) AS total_skills,
            COUNT(DISTINCT CASE WHEN cs.skill_id = ANY(skill_ids) THEN cs.skill_id END) AS matching_skills_count
        FROM 
            guild_app.guilds g
        LEFT JOIN 
            guild_app.character_guilds cg ON g.id = cg.guild_id
        LEFT JOIN 
            guild_app.character_skills cs ON cg.character_id = cs.character_id
        GROUP BY 
            g.id
    )
    SELECT 
        gs.guild_id,
        gs.guild_name,
        gs.description,
        gs.matching_skills_count,
        gs.total_skills,
        gs.quests_completed
    FROM 
        guild_skills gs
    WHERE 
        gs.matching_skills_count > 0
    ORDER BY 
        gs.matching_skills_count DESC,
        gs.quests_completed DESC;
END;
$$ LANGUAGE plpgsql;

-- Function to find guilds in a town
CREATE OR REPLACE FUNCTION guild_app.find_guilds_in_town(town_id UUID)
RETURNS TABLE (
    guild_id UUID,
    guild_name VARCHAR,
    description TEXT,
    guild_master_name VARCHAR,
    member_count INTEGER,
    quests_completed INTEGER
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        g.id AS guild_id,
        g.name AS guild_name,
        g.description,
        c.name AS guild_master_name,
        (SELECT COUNT(*) FROM guild_app.character_guilds WHERE guild_id = g.id) AS member_count,
        g.quests_completed
    FROM 
        guild_app.guilds g
    LEFT JOIN 
        guild_app.characters c ON g.guild_master_id = c.id
    WHERE 
        g.town_id = town_id
    ORDER BY 
        g.quests_completed DESC;
END;
$$ LANGUAGE plpgsql;

-- Function to find available quests for a guild based on skills
CREATE OR REPLACE FUNCTION guild_app.find_available_quests_for_guild(guild_id UUID)
RETURNS TABLE (
    quest_id UUID,
    title VARCHAR,
    description TEXT,
    difficulty INTEGER,
    matching_skills_count INTEGER,
    total_required_skills INTEGER
) AS $$
DECLARE
    guild_skill_ids UUID[];
BEGIN
    -- Get all skills from guild members
    SELECT ARRAY_AGG(DISTINCT cs.skill_id) INTO guild_skill_ids
    FROM guild_app.character_guilds cg
    JOIN guild_app.character_skills cs ON cg.character_id = cs.character_id
    WHERE cg.guild_id = guild_id;
    
    -- If no skills, return empty result
    IF guild_skill_ids IS NULL THEN
        guild_skill_ids := ARRAY[]::UUID[];
    END IF;
    
    -- Get available quests that match the skills
    RETURN QUERY
    WITH quest_skill_matches AS (
        SELECT 
            q.id AS quest_id,
            q.title,
            q.description,
            q.difficulty,
            COUNT(DISTINCT qrs.skill_id) AS total_required_skills,
            COUNT(DISTINCT CASE WHEN qrs.skill_id = ANY(guild_skill_ids) THEN qrs.skill_id END) AS matching_skills_count
        FROM 
            guild_app.quests q
        JOIN 
            guild_app.quest_required_skills qrs ON q.id = qrs.quest_id
        WHERE 
            q.status = 'AVAILABLE'
            AND (q.assigned_guild_id IS NULL OR q.assigned_guild_id = guild_id)
        GROUP BY 
            q.id
    )
    SELECT 
        qsm.quest_id,
        qsm.title,
        qsm.description,
        qsm.difficulty,
        qsm.matching_skills_count,
        qsm.total_required_skills
    FROM 
        quest_skill_matches qsm
    WHERE 
        qsm.matching_skills_count > 0
    ORDER BY 
        (qsm.matching_skills_count::float / qsm.total_required_skills::float) DESC,
        qsm.difficulty ASC;
END;
$$ LANGUAGE plpgsql;

-- Function to find available quests for a character based on skills
CREATE OR REPLACE FUNCTION guild_app.find_available_quests_for_character(character_id UUID, include_guild BOOLEAN DEFAULT FALSE)
RETURNS TABLE (
    quest_id UUID,
    title VARCHAR,
    description TEXT,
    difficulty INTEGER,
    matching_skills_count INTEGER,
    total_required_skills INTEGER
) AS $$
DECLARE
    character_skill_ids UUID[];
    active_guild_id UUID;
    guild_skill_ids UUID[];
BEGIN
    -- Get character skills
    SELECT ARRAY_AGG(skill_id) INTO character_skill_ids
    FROM guild_app.character_skills
    WHERE character_id = character_id;
    
    -- If no skills, return empty result
    IF character_skill_ids IS NULL THEN
        character_skill_ids := ARRAY[]::UUID[];
    END IF;
    
    -- If include_guild is true, add guild skills
    IF include_guild THEN
        -- Get character's active guild
        SELECT c.active_guild_id INTO active_guild_id
        FROM guild_app.characters c
        WHERE c.id = character_id;
        
        -- If character is in a guild, get guild skills
        IF active_guild_id IS NOT NULL THEN
            -- Get skills from all guild members except current character
            SELECT ARRAY_AGG(DISTINCT cs.skill_id) INTO guild_skill_ids
            FROM guild_app.character_guilds cg
            JOIN guild_app.character_skills cs ON cg.character_id = cs.character_id
            WHERE cg.guild_id = active_guild_id
              AND cg.character_id != character_id;
              
            -- Combine character and guild skills
            IF guild_skill_ids IS NOT NULL THEN
                character_skill_ids := character_skill_ids || guild_skill_ids;
            END IF;
        END IF;
    END IF;
    
    -- Get available quests that match the skills
    RETURN QUERY
    WITH quest_skill_matches AS (
        SELECT 
            q.id AS quest_id,
            q.title,
            q.description,
            q.difficulty,
            COUNT(DISTINCT qrs.skill_id) AS total_required_skills,
            COUNT(DISTINCT CASE WHEN qrs.skill_id = ANY(character_skill_ids) THEN qrs.skill_id END) AS matching_skills_count
        FROM 
            guild_app.quests q
        JOIN 
            guild_app.quest_required_skills qrs ON q.id = qrs.quest_id
        WHERE 
            q.status = 'AVAILABLE'
            AND (
                q.assigned_player_id IS NULL 
                OR q.assigned_player_id = character_id
                OR (
                    include_guild = TRUE 
                    AND active_guild_id IS NOT NULL 
                    AND q.assigned_guild_id = active_guild_id
                )
            )
        GROUP BY 
            q.id
    )
    SELECT 
        qsm.quest_id,
        qsm.title,
        qsm.description,
        qsm.difficulty,
        qsm.matching_skills_count,
        qsm.total_required_skills
    FROM 
        quest_skill_matches qsm
    WHERE 
        qsm.matching_skills_count > 0
    ORDER BY 
        (qsm.matching_skills_count::float / qsm.total_required_skills::float) DESC,
        qsm.difficulty ASC;
END;
$$ LANGUAGE plpgsql; 