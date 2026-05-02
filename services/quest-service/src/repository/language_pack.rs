use tokio_postgres::{Client, Row};
use uuid::Uuid;
use chrono::Utc;
use log::{info, error};
use crate::models::language_pack::{
    LanguagePack, LanguagePackCreate, LanguagePackUpdate, 
    TerminologySet, QuestTypes
};
use crate::repository::db::DbError;

pub struct LanguagePackRepository {
    client: Client,
}

impl LanguagePackRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
    
    pub async fn get_language_packs(
        &self,
        offset: i64,
        limit: i64,
        language_code: Option<&str>,
    ) -> Result<(Vec<LanguagePack>, i64), DbError> {
        let mut query = String::from(
            "SELECT id, language_code, language_name, is_current, created_at, updated_at,
                real_quest_term, real_quest_reward_term, real_epic_term, real_adventure_term, 
                real_character_term, real_guild_term, real_skill_term,
                real_design_term, real_proof_of_concept_term, real_implement_term, 
                real_bug_check_term, real_subdivide_term,
                game_quest_term, game_quest_reward_term, game_epic_term, game_adventure_term, 
                game_character_term, game_guild_term, game_skill_term,
                game_design_term, game_proof_of_concept_term, game_implement_term, 
                game_bug_check_term, game_subdivide_term
            FROM language_packs 
            WHERE 1=1"
        );
        
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        let mut param_count = 1;
        
        if let Some(code) = language_code {
            query.push_str(&format!(" AND language_code = ${}", param_count));
            params.push(&code);
            param_count += 1;
        }
        
        // Count total before applying pagination
        let count_query = format!("SELECT COUNT(*) FROM ({}) as count_query", query);
        let count_row = self.client.query_one(
            &count_query,
            &params[..],
        ).await.map_err(DbError::from)?;
        
        let total: i64 = count_row.get(0);
        
        // Apply pagination
        query.push_str(&format!(" ORDER BY language_name ASC LIMIT ${} OFFSET ${}", 
            param_count, param_count + 1));
            
        params.push(&limit);
        params.push(&offset);
        
        let rows = self.client.query(
            &query,
            &params[..],
        ).await.map_err(DbError::from)?;
        
        let language_packs = rows.iter()
            .map(|row| self.row_to_language_pack(row))
            .collect();
            
        Ok((language_packs, total))
    }
    
    pub async fn get_language_pack_by_id(&self, id: Uuid) -> Result<LanguagePack, DbError> {
        let row = self.client.query_one(
            "SELECT id, language_code, language_name, is_current, created_at, updated_at,
                real_quest_term, real_quest_reward_term, real_epic_term, real_adventure_term, 
                real_character_term, real_guild_term, real_skill_term,
                real_design_term, real_proof_of_concept_term, real_implement_term, 
                real_bug_check_term, real_subdivide_term,
                game_quest_term, game_quest_reward_term, game_epic_term, game_adventure_term, 
                game_character_term, game_guild_term, game_skill_term,
                game_design_term, game_proof_of_concept_term, game_implement_term, 
                game_bug_check_term, game_subdivide_term
            FROM language_packs 
            WHERE id = $1",
            &[&id],
        ).await.map_err(|e| {
            if e.to_string().contains("no rows") {
                DbError::NoDataReturned
            } else {
                DbError::from(e)
            }
        })?;
        
        Ok(self.row_to_language_pack(&row))
    }
    
    pub async fn get_current_language_pack(&self) -> Result<LanguagePack, DbError> {
        let row = self.client.query_one(
            "SELECT id, language_code, language_name, is_current, created_at, updated_at,
                real_quest_term, real_quest_reward_term, real_epic_term, real_adventure_term, 
                real_character_term, real_guild_term, real_skill_term,
                real_design_term, real_proof_of_concept_term, real_implement_term, 
                real_bug_check_term, real_subdivide_term,
                game_quest_term, game_quest_reward_term, game_epic_term, game_adventure_term, 
                game_character_term, game_guild_term, game_skill_term,
                game_design_term, game_proof_of_concept_term, game_implement_term, 
                game_bug_check_term, game_subdivide_term
            FROM language_packs 
            WHERE is_current = true",
            &[],
        ).await.map_err(|e| {
            if e.to_string().contains("no rows") {
                DbError::NoDataReturned
            } else {
                DbError::from(e)
            }
        })?;
        
        Ok(self.row_to_language_pack(&row))
    }
    
    pub async fn create_language_pack(&self, pack: LanguagePackCreate) -> Result<LanguagePack, DbError> {
        // Start a transaction since we might need to update multiple rows
        let tx = self.client.transaction().await.map_err(DbError::from)?;
        
        // If this is set as current, update all other language packs to not be current
        if pack.is_current.unwrap_or(false) {
            tx.execute(
                "UPDATE language_packs SET is_current = false",
                &[],
            ).await.map_err(DbError::from)?;
        }
        
        // Insert the new language pack
        let row = tx.query_one(
            "INSERT INTO language_packs (
                language_code, language_name, is_current, created_at, updated_at,
                real_quest_term, real_quest_reward_term, real_epic_term, real_adventure_term, 
                real_character_term, real_guild_term, real_skill_term,
                real_design_term, real_proof_of_concept_term, real_implement_term, 
                real_bug_check_term, real_subdivide_term,
                game_quest_term, game_quest_reward_term, game_epic_term, game_adventure_term, 
                game_character_term, game_guild_term, game_skill_term,
                game_design_term, game_proof_of_concept_term, game_implement_term, 
                game_bug_check_term, game_subdivide_term
            ) VALUES (
                $1, $2, $3, $4, $4,
                $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16,
                $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28
            )
            RETURNING id, language_code, language_name, is_current, created_at, updated_at,
                real_quest_term, real_quest_reward_term, real_epic_term, real_adventure_term, 
                real_character_term, real_guild_term, real_skill_term,
                real_design_term, real_proof_of_concept_term, real_implement_term, 
                real_bug_check_term, real_subdivide_term,
                game_quest_term, game_quest_reward_term, game_epic_term, game_adventure_term, 
                game_character_term, game_guild_term, game_skill_term,
                game_design_term, game_proof_of_concept_term, game_implement_term, 
                game_bug_check_term, game_subdivide_term",
            &[
                &pack.language_code,
                &pack.language_name,
                &pack.is_current.unwrap_or(false),
                &Utc::now(),
                
                // Real-world terminology
                &pack.real_world.quest_term,
                &pack.real_world.quest_reward_term,
                &pack.real_world.epic_term,
                &pack.real_world.adventure_term,
                &pack.real_world.character_term,
                &pack.real_world.guild_term,
                &pack.real_world.skill_term,
                &pack.real_world.quest_types.design,
                &pack.real_world.quest_types.proof_of_concept,
                &pack.real_world.quest_types.implement,
                &pack.real_world.quest_types.bug_check,
                &pack.real_world.quest_types.subdivide,
                
                // In-game terminology
                &pack.in_game.quest_term,
                &pack.in_game.quest_reward_term,
                &pack.in_game.epic_term,
                &pack.in_game.adventure_term,
                &pack.in_game.character_term,
                &pack.in_game.guild_term,
                &pack.in_game.skill_term,
                &pack.in_game.quest_types.design,
                &pack.in_game.quest_types.proof_of_concept,
                &pack.in_game.quest_types.implement,
                &pack.in_game.quest_types.bug_check,
                &pack.in_game.quest_types.subdivide,
            ],
        ).await.map_err(DbError::from)?;
        
        // Commit the transaction
        tx.commit().await.map_err(DbError::from)?;
        
        Ok(self.row_to_language_pack(&row))
    }
    
    pub async fn update_language_pack(&self, id: Uuid, pack: LanguagePackUpdate) -> Result<LanguagePack, DbError> {
        // First check if the language pack exists
        let _ = self.get_language_pack_by_id(id).await?;
        
        // Start a transaction since we might need to update multiple rows
        let tx = self.client.transaction().await.map_err(DbError::from)?;
        
        // If this is being set as current, update all other language packs to not be current
        if let Some(is_current) = pack.is_current {
            if is_current {
                tx.execute(
                    "UPDATE language_packs SET is_current = false",
                    &[],
                ).await.map_err(DbError::from)?;
            }
        }
        
        // Build the update query
        let mut query = String::from("UPDATE language_packs SET updated_at = $1");
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        
        let now = Utc::now();
        params.push(&now);
        
        let mut param_count = 2;
        
        if let Some(language_code) = &pack.language_code {
            query.push_str(&format!(", language_code = ${}", param_count));
            params.push(language_code);
            param_count += 1;
        }
        
        if let Some(language_name) = &pack.language_name {
            query.push_str(&format!(", language_name = ${}", param_count));
            params.push(language_name);
            param_count += 1;
        }
        
        if let Some(is_current) = &pack.is_current {
            query.push_str(&format!(", is_current = ${}", param_count));
            params.push(is_current);
            param_count += 1;
        }
        
        // Update real-world terminology if provided
        if let Some(real_world) = &pack.real_world {
            if let Some(quest_term) = &real_world.quest_term {
                query.push_str(&format!(", real_quest_term = ${}", param_count));
                params.push(quest_term);
                param_count += 1;
            }
            
            if let Some(quest_reward_term) = &real_world.quest_reward_term {
                query.push_str(&format!(", real_quest_reward_term = ${}", param_count));
                params.push(quest_reward_term);
                param_count += 1;
            }
            
            if let Some(epic_term) = &real_world.epic_term {
                query.push_str(&format!(", real_epic_term = ${}", param_count));
                params.push(epic_term);
                param_count += 1;
            }
            
            if let Some(adventure_term) = &real_world.adventure_term {
                query.push_str(&format!(", real_adventure_term = ${}", param_count));
                params.push(adventure_term);
                param_count += 1;
            }
            
            if let Some(character_term) = &real_world.character_term {
                query.push_str(&format!(", real_character_term = ${}", param_count));
                params.push(character_term);
                param_count += 1;
            }
            
            if let Some(guild_term) = &real_world.guild_term {
                query.push_str(&format!(", real_guild_term = ${}", param_count));
                params.push(guild_term);
                param_count += 1;
            }
            
            if let Some(skill_term) = &real_world.skill_term {
                query.push_str(&format!(", real_skill_term = ${}", param_count));
                params.push(skill_term);
                param_count += 1;
            }
            
            // Update quest types if provided
            if let Some(quest_types) = &real_world.quest_types {
                if let Some(design) = &quest_types.design {
                    query.push_str(&format!(", real_design_term = ${}", param_count));
                    params.push(design);
                    param_count += 1;
                }
                
                if let Some(proof_of_concept) = &quest_types.proof_of_concept {
                    query.push_str(&format!(", real_proof_of_concept_term = ${}", param_count));
                    params.push(proof_of_concept);
                    param_count += 1;
                }
                
                if let Some(implement) = &quest_types.implement {
                    query.push_str(&format!(", real_implement_term = ${}", param_count));
                    params.push(implement);
                    param_count += 1;
                }
                
                if let Some(bug_check) = &quest_types.bug_check {
                    query.push_str(&format!(", real_bug_check_term = ${}", param_count));
                    params.push(bug_check);
                    param_count += 1;
                }
                
                if let Some(subdivide) = &quest_types.subdivide {
                    query.push_str(&format!(", real_subdivide_term = ${}", param_count));
                    params.push(subdivide);
                    param_count += 1;
                }
            }
        }
        
        // Update in-game terminology if provided
        if let Some(in_game) = &pack.in_game {
            if let Some(quest_term) = &in_game.quest_term {
                query.push_str(&format!(", game_quest_term = ${}", param_count));
                params.push(quest_term);
                param_count += 1;
            }
            
            if let Some(quest_reward_term) = &in_game.quest_reward_term {
                query.push_str(&format!(", game_quest_reward_term = ${}", param_count));
                params.push(quest_reward_term);
                param_count += 1;
            }
            
            if let Some(epic_term) = &in_game.epic_term {
                query.push_str(&format!(", game_epic_term = ${}", param_count));
                params.push(epic_term);
                param_count += 1;
            }
            
            if let Some(adventure_term) = &in_game.adventure_term {
                query.push_str(&format!(", game_adventure_term = ${}", param_count));
                params.push(adventure_term);
                param_count += 1;
            }
            
            if let Some(character_term) = &in_game.character_term {
                query.push_str(&format!(", game_character_term = ${}", param_count));
                params.push(character_term);
                param_count += 1;
            }
            
            if let Some(guild_term) = &in_game.guild_term {
                query.push_str(&format!(", game_guild_term = ${}", param_count));
                params.push(guild_term);
                param_count += 1;
            }
            
            if let Some(skill_term) = &in_game.skill_term {
                query.push_str(&format!(", game_skill_term = ${}", param_count));
                params.push(skill_term);
                param_count += 1;
            }
            
            // Update quest types if provided
            if let Some(quest_types) = &in_game.quest_types {
                if let Some(design) = &quest_types.design {
                    query.push_str(&format!(", game_design_term = ${}", param_count));
                    params.push(design);
                    param_count += 1;
                }
                
                if let Some(proof_of_concept) = &quest_types.proof_of_concept {
                    query.push_str(&format!(", game_proof_of_concept_term = ${}", param_count));
                    params.push(proof_of_concept);
                    param_count += 1;
                }
                
                if let Some(implement) = &quest_types.implement {
                    query.push_str(&format!(", game_implement_term = ${}", param_count));
                    params.push(implement);
                    param_count += 1;
                }
                
                if let Some(bug_check) = &quest_types.bug_check {
                    query.push_str(&format!(", game_bug_check_term = ${}", param_count));
                    params.push(bug_check);
                    param_count += 1;
                }
                
                if let Some(subdivide) = &quest_types.subdivide {
                    query.push_str(&format!(", game_subdivide_term = ${}", param_count));
                    params.push(subdivide);
                    param_count += 1;
                }
            }
        }
        
        // Add WHERE clause and RETURNING
        query.push_str(&format!(" WHERE id = ${} 
            RETURNING id, language_code, language_name, is_current, created_at, updated_at,
                real_quest_term, real_quest_reward_term, real_epic_term, real_adventure_term, 
                real_character_term, real_guild_term, real_skill_term,
                real_design_term, real_proof_of_concept_term, real_implement_term, 
                real_bug_check_term, real_subdivide_term,
                game_quest_term, game_quest_reward_term, game_epic_term, game_adventure_term, 
                game_character_term, game_guild_term, game_skill_term,
                game_design_term, game_proof_of_concept_term, game_implement_term, 
                game_bug_check_term, game_subdivide_term", 
            param_count
        ));
            
        params.push(&id);
        
        // Execute the update
        let row = tx.query_one(
            &query,
            &params[..],
        ).await.map_err(DbError::from)?;
        
        // Commit the transaction
        tx.commit().await.map_err(DbError::from)?;
        
        Ok(self.row_to_language_pack(&row))
    }
    
    pub async fn set_current_language_pack(&self, id: Uuid) -> Result<(), DbError> {
        // Start a transaction
        let tx = self.client.transaction().await.map_err(DbError::from)?;
        
        // First set all language packs to not current
        tx.execute(
            "UPDATE language_packs SET is_current = false",
            &[],
        ).await.map_err(DbError::from)?;
        
        // Then set the specified one as current
        let result = tx.execute(
            "UPDATE language_packs SET is_current = true, updated_at = $1 WHERE id = $2",
            &[&Utc::now(), &id],
        ).await.map_err(DbError::from)?;
        
        if result == 0 {
            tx.rollback().await.map_err(DbError::from)?;
            return Err(DbError::NoDataReturned);
        }
        
        // Commit the transaction
        tx.commit().await.map_err(DbError::from)?;
        
        Ok(())
    }
    
    pub async fn delete_language_pack(&self, id: Uuid) -> Result<(), DbError> {
        // Check if it's the current language pack
        let lang_pack = self.get_language_pack_by_id(id).await?;
        if lang_pack.is_current {
            return Err(DbError::Other(
                "Cannot delete the current active language pack".to_string()
            ));
        }
        
        let result = self.client.execute(
            "DELETE FROM language_packs WHERE id = $1",
            &[&id],
        ).await.map_err(DbError::from)?;
        
        if result == 0 {
            return Err(DbError::NoDataReturned);
        }
        
        Ok(())
    }
    
    // Helper function to convert a Row to a LanguagePack
    fn row_to_language_pack(&self, row: &Row) -> LanguagePack {
        // Create real-world terminology set
        let real_world = TerminologySet {
            quest_term: row.get("real_quest_term"),
            quest_reward_term: row.get("real_quest_reward_term"),
            epic_term: row.get("real_epic_term"),
            adventure_term: row.get("real_adventure_term"),
            character_term: row.get("real_character_term"),
            guild_term: row.get("real_guild_term"),
            skill_term: row.get("real_skill_term"),
            quest_types: QuestTypes {
                design: row.get("real_design_term"),
                proof_of_concept: row.get("real_proof_of_concept_term"),
                implement: row.get("real_implement_term"),
                bug_check: row.get("real_bug_check_term"),
                subdivide: row.get("real_subdivide_term"),
            },
        };
        
        // Create in-game terminology set
        let in_game = TerminologySet {
            quest_term: row.get("game_quest_term"),
            quest_reward_term: row.get("game_quest_reward_term"),
            epic_term: row.get("game_epic_term"),
            adventure_term: row.get("game_adventure_term"),
            character_term: row.get("game_character_term"),
            guild_term: row.get("game_guild_term"),
            skill_term: row.get("game_skill_term"),
            quest_types: QuestTypes {
                design: row.get("game_design_term"),
                proof_of_concept: row.get("game_proof_of_concept_term"),
                implement: row.get("game_implement_term"),
                bug_check: row.get("game_bug_check_term"),
                subdivide: row.get("game_subdivide_term"),
            },
        };
        
        LanguagePack {
            id: row.get("id"),
            language_code: row.get("language_code"),
            language_name: row.get("language_name"),
            is_current: row.get("is_current"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            real_world,
            in_game,
        }
    }
} 