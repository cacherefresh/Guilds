-- Create guild schema if it doesn't exist
CREATE SCHEMA IF NOT EXISTS guild_app;

-- Create town table
CREATE TABLE IF NOT EXISTS guild_app.towns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    region VARCHAR(255) NOT NULL,
    properties JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create guild table
CREATE TABLE IF NOT EXISTS guild_app.guilds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    guild_master_id UUID REFERENCES guild_app.characters(id),
    town_id UUID REFERENCES guild_app.towns(id),
    reward_divider_percentage INTEGER NOT NULL DEFAULT 80 CHECK (reward_divider_percentage BETWEEN 0 AND 100),
    stash_reward INTEGER NOT NULL DEFAULT 0 CHECK (stash_reward >= 0),
    quests_completed INTEGER NOT NULL DEFAULT 0 CHECK (quests_completed >= 0),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create guild membership table
CREATE TABLE IF NOT EXISTS guild_app.character_guilds (
    character_id UUID NOT NULL REFERENCES guild_app.characters(id) ON DELETE CASCADE,
    guild_id UUID NOT NULL REFERENCES guild_app.guilds(id) ON DELETE CASCADE,
    is_active BOOLEAN NOT NULL DEFAULT FALSE,
    joined_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (character_id, guild_id)
);

-- Create AI teammate table
CREATE TABLE IF NOT EXISTS guild_app.ai_teammates (
    guild_id UUID NOT NULL REFERENCES guild_app.guilds(id) ON DELETE CASCADE,
    character_id UUID NOT NULL REFERENCES guild_app.characters(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (guild_id)
);

-- Add guild-related fields to characters table
ALTER TABLE guild_app.characters
ADD COLUMN IF NOT EXISTS active_guild_id UUID REFERENCES guild_app.guilds(id);

-- Add guild-related fields to quests table
ALTER TABLE guild_app.quests
ADD COLUMN IF NOT EXISTS assigned_guild_id UUID REFERENCES guild_app.guilds(id),
ADD COLUMN IF NOT EXISTS contact_character_id UUID REFERENCES guild_app.characters(id);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_character_guilds_character_id ON guild_app.character_guilds(character_id);
CREATE INDEX IF NOT EXISTS idx_character_guilds_guild_id ON guild_app.character_guilds(guild_id);
CREATE INDEX IF NOT EXISTS idx_character_guilds_is_active ON guild_app.character_guilds(is_active);
CREATE INDEX IF NOT EXISTS idx_characters_active_guild_id ON guild_app.characters(active_guild_id);
CREATE INDEX IF NOT EXISTS idx_guilds_guild_master_id ON guild_app.guilds(guild_master_id);
CREATE INDEX IF NOT EXISTS idx_guilds_town_id ON guild_app.guilds(town_id);
CREATE INDEX IF NOT EXISTS idx_quests_assigned_guild_id ON guild_app.quests(assigned_guild_id);
CREATE INDEX IF NOT EXISTS idx_quests_contact_character_id ON guild_app.quests(contact_character_id);
CREATE INDEX IF NOT EXISTS idx_towns_region ON guild_app.towns(region);

-- Create helper functions
CREATE OR REPLACE FUNCTION guild_app.find_guilds_by_skills(skill_ids UUID[])
RETURNS TABLE (
    guild_id UUID,
    guild_name VARCHAR(255),
    description TEXT,
    matching_skills_count BIGINT,
    total_skills BIGINT,
    quests_completed INTEGER
) AS $$
BEGIN
    RETURN QUERY
    WITH guild_skills AS (
        SELECT 
            g.id as guild_id,
            COUNT(DISTINCT cs.skill_id) FILTER (WHERE cs.skill_id = ANY(skill_ids)) as matching_count,
            COUNT(DISTINCT cs.skill_id) as total_count
        FROM 
            guild_app.guilds g
        JOIN 
            guild_app.character_guilds cg ON g.id = cg.guild_id
        JOIN 
            guild_app.character_skills cs ON cg.character_id = cs.character_id
        GROUP BY 
            g.id
    )
    SELECT 
        g.id as guild_id,
        g.name as guild_name,
        g.description,
        COALESCE(gs.matching_count, 0) as matching_skills_count,
        COALESCE(gs.total_count, 0) as total_skills,
        g.quests_completed
    FROM 
        guild_app.guilds g
    LEFT JOIN 
        guild_skills gs ON g.id = gs.guild_id
    WHERE 
        COALESCE(gs.matching_count, 0) > 0
    ORDER BY 
        gs.matching_count DESC,
        g.quests_completed DESC;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION guild_app.find_guilds_in_town(p_town_id UUID)
RETURNS TABLE (
    guild_id UUID,
    guild_name VARCHAR(255),
    description TEXT,
    guild_master_name VARCHAR(255),
    member_count BIGINT,
    quests_completed INTEGER
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        g.id as guild_id,
        g.name as guild_name,
        g.description,
        c.name as guild_master_name,
        COUNT(DISTINCT cg.character_id) as member_count,
        g.quests_completed
    FROM 
        guild_app.guilds g
    LEFT JOIN 
        guild_app.characters c ON g.guild_master_id = c.id
    LEFT JOIN 
        guild_app.character_guilds cg ON g.id = cg.guild_id
    WHERE 
        g.town_id = p_town_id
    GROUP BY 
        g.id, c.name
    ORDER BY 
        g.name ASC;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to ensure character can only be in 3 guilds max
CREATE OR REPLACE FUNCTION guild_app.check_guild_limit()
RETURNS TRIGGER AS $$
DECLARE
    guild_count INTEGER;
BEGIN
    SELECT COUNT(*) INTO guild_count
    FROM guild_app.character_guilds
    WHERE character_id = NEW.character_id;
    
    IF guild_count >= 3 AND TG_OP = 'INSERT' THEN
        RAISE EXCEPTION 'Character can only be a member of up to 3 guilds';
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_guild_limit_trigger
BEFORE INSERT ON guild_app.character_guilds
FOR EACH ROW
EXECUTE FUNCTION guild_app.check_guild_limit();

-- Create trigger to add guild master as member when creating a guild
CREATE OR REPLACE FUNCTION guild_app.add_guild_master_as_member()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.guild_master_id IS NOT NULL THEN
        INSERT INTO guild_app.character_guilds (character_id, guild_id, is_active)
        VALUES (NEW.guild_master_id, NEW.id, TRUE)
        ON CONFLICT (character_id, guild_id) DO UPDATE
        SET is_active = TRUE;
        
        -- Update character's active guild
        UPDATE guild_app.characters
        SET active_guild_id = NEW.id
        WHERE id = NEW.guild_master_id;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER add_guild_master_trigger
AFTER INSERT ON guild_app.guilds
FOR EACH ROW
EXECUTE FUNCTION guild_app.add_guild_master_as_member();

-- Create trigger to ensure only one guild is active per character
CREATE OR REPLACE FUNCTION guild_app.ensure_one_active_guild()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.is_active = TRUE THEN
        -- Set all other guild memberships to inactive
        UPDATE guild_app.character_guilds
        SET is_active = FALSE
        WHERE character_id = NEW.character_id
        AND guild_id != NEW.guild_id;
        
        -- Update the character's active guild
        UPDATE guild_app.characters
        SET active_guild_id = NEW.guild_id
        WHERE id = NEW.character_id;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER ensure_one_active_guild_trigger
AFTER INSERT OR UPDATE ON guild_app.character_guilds
FOR EACH ROW
WHEN (NEW.is_active = TRUE)
EXECUTE FUNCTION guild_app.ensure_one_active_guild();

-- Create trigger to update timestamps
CREATE OR REPLACE FUNCTION guild_app.update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_guilds_timestamp
BEFORE UPDATE ON guild_app.guilds
FOR EACH ROW
EXECUTE FUNCTION guild_app.update_modified_column();

CREATE TRIGGER update_towns_timestamp
BEFORE UPDATE ON guild_app.towns
FOR EACH ROW
EXECUTE FUNCTION guild_app.update_modified_column();

-- Create sample towns
INSERT INTO guild_app.towns (name, description, region, properties)
VALUES
    ('Waterdeep', 'A bustling coastal city known for its commerce and adventuring opportunities', 'Sword Coast', '{"population": 100000, "specialty": "commerce", "government": "council"}'),
    ('Neverwinter', 'A city rebuilt after volcanic disaster, known for its skilled craftsmen', 'Sword Coast', '{"population": 50000, "specialty": "crafting", "government": "lord"}'),
    ('Baldur''s Gate', 'A fortified city with a strong military presence and trade connections', 'Sword Coast', '{"population": 80000, "specialty": "trade", "government": "council"}'),
    ('Silverymoon', 'A magical city with a large population of spellcasters and magical institutions', 'Silver Marches', '{"population": 40000, "specialty": "magic", "government": "high mage"}'),
    ('Mirabar', 'A mining city rich in precious metals and gems', 'Silver Marches', '{"population": 35000, "specialty": "mining", "government": "marchion"}'); 