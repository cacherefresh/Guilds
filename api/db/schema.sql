-- Create extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create schema
CREATE SCHEMA IF NOT EXISTS guild_app;

-- Set search path
SET search_path TO guild_app, public;

-- Skills table
CREATE TABLE IF NOT EXISTS skills (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    category VARCHAR(50) NOT NULL CHECK (category IN ('COMBAT', 'MAGIC', 'CRAFTING', 'SOCIAL', 'STEALTH', 'KNOWLEDGE')),
    level INTEGER NOT NULL DEFAULT 1 CHECK (level >= 1 AND level <= 100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Team table
CREATE TABLE IF NOT EXISTS teams (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    guild VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Characters table
CREATE TABLE IF NOT EXISTS characters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    type VARCHAR(50) NOT NULL CHECK (type IN ('MAIN', 'TEAMMATE', 'SHADOW_CLONE', 'SHADOW_MINION')),
    level INTEGER NOT NULL DEFAULT 1,
    xp INTEGER NOT NULL DEFAULT 0,
    guild VARCHAR(100),
    team_id UUID REFERENCES teams(id) ON DELETE SET NULL,
    properties JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Character Skills junction table
CREATE TABLE IF NOT EXISTS character_skills (
    character_id UUID REFERENCES characters(id) ON DELETE CASCADE,
    skill_id UUID REFERENCES skills(id) ON DELETE CASCADE,
    proficiency INTEGER NOT NULL DEFAULT 1 CHECK (proficiency >= 1 AND proficiency <= 100),
    PRIMARY KEY (character_id, skill_id)
);

-- Quests table
CREATE TABLE IF NOT EXISTS quests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(200) NOT NULL,
    description TEXT NOT NULL,
    difficulty INTEGER NOT NULL DEFAULT 1 CHECK (difficulty >= 1 AND difficulty <= 10),
    reward TEXT,
    xp_reward INTEGER NOT NULL DEFAULT 0,
    gold_reward INTEGER NOT NULL DEFAULT 0,
    status VARCHAR(50) NOT NULL DEFAULT 'AVAILABLE' CHECK (status IN ('AVAILABLE', 'ASSIGNED', 'COMPLETED', 'FAILED')),
    assigned_to_character_id UUID REFERENCES characters(id) ON DELETE SET NULL,
    completed_by_character_id UUID REFERENCES characters(id) ON DELETE SET NULL,
    completion_date TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Quest Skills junction table (required skills for a quest)
CREATE TABLE IF NOT EXISTS quest_required_skills (
    quest_id UUID REFERENCES quests(id) ON DELETE CASCADE,
    skill_id UUID REFERENCES skills(id) ON DELETE CASCADE,
    minimum_level INTEGER NOT NULL DEFAULT 1 CHECK (minimum_level >= 1 AND minimum_level <= 100),
    PRIMARY KEY (quest_id, skill_id)
);

-- Quest Tracking table (for recurring quests, history, etc.)
CREATE TABLE IF NOT EXISTS quest_tracking (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    quest_id UUID REFERENCES quests(id) ON DELETE CASCADE,
    character_id UUID REFERENCES characters(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL CHECK (status IN ('STARTED', 'ABANDONED', 'COMPLETED', 'FAILED')),
    started_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    notes TEXT,
    rewards_claimed BOOLEAN DEFAULT FALSE
);

-- Add triggers for updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger for each table
CREATE TRIGGER update_skills_updated_at
BEFORE UPDATE ON skills
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER update_teams_updated_at
BEFORE UPDATE ON teams
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER update_characters_updated_at
BEFORE UPDATE ON characters
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER update_quests_updated_at
BEFORE UPDATE ON quests
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

-- Indexes for performance
CREATE INDEX idx_character_team ON characters(team_id);
CREATE INDEX idx_quest_assigned_to ON quests(assigned_to_character_id);
CREATE INDEX idx_quest_completed_by ON quests(completed_by_character_id);
CREATE INDEX idx_character_skills_char ON character_skills(character_id);
CREATE INDEX idx_character_skills_skill ON character_skills(skill_id);
CREATE INDEX idx_quest_required_skills_quest ON quest_required_skills(quest_id);
CREATE INDEX idx_quest_required_skills_skill ON quest_required_skills(skill_id);
CREATE INDEX idx_quest_tracking_quest ON quest_tracking(quest_id);
CREATE INDEX idx_quest_tracking_character ON quest_tracking(character_id);

-- Function to find available quests for a character based on skills
CREATE OR REPLACE FUNCTION find_available_quests_for_character(character_uuid UUID, include_team BOOLEAN DEFAULT FALSE)
RETURNS TABLE (
    quest_id UUID,
    title VARCHAR,
    description TEXT,
    difficulty INTEGER,
    matching_skills_count INTEGER,
    total_required_skills INTEGER
) AS $$
DECLARE
    character_team_id UUID;
BEGIN
    -- Get the character's team ID if include_team is true
    IF include_team THEN
        SELECT team_id INTO character_team_id FROM characters WHERE id = character_uuid;
    END IF;

    RETURN QUERY
    WITH character_skills AS (
        -- Get all skills from the character
        SELECT cs.skill_id, cs.proficiency
        FROM character_skills cs
        WHERE cs.character_id = character_uuid
        
        UNION
        
        -- Include team members' skills if requested
        SELECT cs.skill_id, cs.proficiency
        FROM character_skills cs
        JOIN characters c ON cs.character_id = c.id
        WHERE include_team = TRUE
          AND character_team_id IS NOT NULL
          AND c.team_id = character_team_id
          AND c.id != character_uuid
    ),
    quest_skills AS (
        -- Count required skills for each quest
        SELECT 
            q.id AS quest_id,
            q.title,
            q.description,
            q.difficulty,
            COUNT(qrs.skill_id) AS total_required_skills,
            COUNT(cs.skill_id) AS matching_skills_count
        FROM quests q
        JOIN quest_required_skills qrs ON q.id = qrs.quest_id
        LEFT JOIN character_skills cs ON qrs.skill_id = cs.skill_id 
                                     AND cs.proficiency >= qrs.minimum_level
        WHERE q.status = 'AVAILABLE'
        GROUP BY q.id, q.title, q.description, q.difficulty
    )
    SELECT 
        qs.quest_id,
        qs.title,
        qs.description,
        qs.difficulty,
        qs.matching_skills_count,
        qs.total_required_skills
    FROM quest_skills qs
    WHERE qs.matching_skills_count > 0
    ORDER BY 
        -- Sort by percentage of matching skills (descending)
        (qs.matching_skills_count::float / qs.total_required_skills::float) DESC,
        -- Then by difficulty (ascending)
        qs.difficulty ASC;
END;
$$ LANGUAGE plpgsql;

-- Function to find quests that match exactly the provided skills
CREATE OR REPLACE FUNCTION find_quests_by_exact_skills(skill_ids UUID[])
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
            COUNT(qrs.skill_id) AS total_required_skills,
            COUNT(CASE WHEN qrs.skill_id = ANY(skill_ids) THEN 1 END) AS matching_skills_count
        FROM quests q
        JOIN quest_required_skills qrs ON q.id = qrs.quest_id
        WHERE q.status = 'AVAILABLE'
        GROUP BY q.id, q.title, q.description, q.difficulty
    )
    SELECT 
        qsc.quest_id,
        qsc.title,
        qsc.description,
        qsc.difficulty
    FROM quest_skill_counts qsc
    WHERE qsc.matching_skills_count = qsc.total_required_skills
      AND qsc.total_required_skills = array_length(skill_ids, 1)
    ORDER BY qsc.difficulty ASC;
END;
$$ LANGUAGE plpgsql;

-- Function to find quests that require a subset of the provided skills
CREATE OR REPLACE FUNCTION find_quests_by_subset_skills(skill_ids UUID[])
RETURNS TABLE (
    quest_id UUID,
    title VARCHAR,
    description TEXT,
    difficulty INTEGER,
    matching_skills_count INTEGER,
    total_required_skills INTEGER
) AS $$
BEGIN
    RETURN QUERY
    WITH quest_skill_counts AS (
        SELECT 
            q.id AS quest_id,
            q.title,
            q.description,
            q.difficulty,
            COUNT(qrs.skill_id) AS total_required_skills,
            COUNT(CASE WHEN qrs.skill_id = ANY(skill_ids) THEN 1 END) AS matching_skills_count
        FROM quests q
        JOIN quest_required_skills qrs ON q.id = qrs.quest_id
        WHERE q.status = 'AVAILABLE'
        GROUP BY q.id, q.title, q.description, q.difficulty
    )
    SELECT 
        qsc.quest_id,
        qsc.title,
        qsc.description,
        qsc.difficulty,
        qsc.matching_skills_count,
        qsc.total_required_skills
    FROM quest_skill_counts qsc
    WHERE qsc.matching_skills_count > 0
    ORDER BY 
        -- Sort by percentage of matching skills (descending)
        (qsc.matching_skills_count::float / qsc.total_required_skills::float) DESC,
        -- Then by difficulty (ascending)
        qsc.difficulty ASC;
END;
$$ LANGUAGE plpgsql; 