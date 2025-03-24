use crate::error::AppError;
use crate::models::character::{Character, CharacterLocationDto, CreateCharacterDto, UpdateCharacterDto};
use sqlx::{Pool, Postgres};
use sqlx::types::Uuid;

pub async fn get_all(pool: &Pool<Postgres>) -> Result<Vec<Character>, AppError> {
    let characters = sqlx::query_as!(
        Character,
        r#"
        SELECT id, name, guild_name, interests, skills, magic_abilities, 
               position_x, position_y, position_z, current_action
        FROM characters
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(characters)
}

pub async fn get_by_id(pool: &Pool<Postgres>, id: Uuid) -> Result<Character, AppError> {
    let character = sqlx::query_as!(
        Character,
        r#"
        SELECT id, name, guild_name, interests, skills, magic_abilities, 
               position_x, position_y, position_z, current_action
        FROM characters
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFoundError(format!("Character with ID {} not found", id)),
        _ => AppError::DatabaseError(e),
    })?;

    Ok(character)
}

pub async fn create(pool: &Pool<Postgres>, dto: CreateCharacterDto) -> Result<Character, AppError> {
    let id = Uuid::new_v4();
    let guild_name = dto.guild_name.unwrap_or_else(|| "Default Guild".to_string());
    
    // Convert Vec<String> to array for PostgreSQL
    let interests = dto.interests.unwrap_or_else(|| vec!["Music".to_string(), "Programming".to_string(), "Bringing AI to Life".to_string()]);
    let skills = dto.skills.unwrap_or_else(Vec::new);
    let magic_abilities = dto.magic_abilities.unwrap_or_else(|| vec!["Shadow Clone".to_string(), "Shadow Minion".to_string()]);

    let character = sqlx::query_as!(
        Character,
        r#"
        INSERT INTO characters (id, name, guild_name, interests, skills, magic_abilities, 
                             position_x, position_y, position_z, current_action)
        VALUES ($1, $2, $3, $4, $5, $6, 0, 0, 0, 'idle')
        RETURNING id, name, guild_name, interests, skills, magic_abilities, 
                  position_x, position_y, position_z, current_action
        "#,
        id,
        dto.name,
        guild_name,
        &interests as _,
        &skills as _,
        &magic_abilities as _
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(character)
}

pub async fn update(pool: &Pool<Postgres>, id: Uuid, dto: UpdateCharacterDto) -> Result<Character, AppError> {
    // First, check if the character exists
    let character = get_by_id(pool, id).await?;

    // Update only the fields that are provided
    let name = dto.name.unwrap_or(character.name);
    let guild_name = dto.guild_name.unwrap_or(character.guild_name);
    let interests = dto.interests.unwrap_or(character.interests);
    let skills = dto.skills.unwrap_or(character.skills);
    let magic_abilities = dto.magic_abilities.unwrap_or(character.magic_abilities);
    let current_action = dto.current_action.unwrap_or(character.current_action);

    let updated_character = sqlx::query_as!(
        Character,
        r#"
        UPDATE characters
        SET name = $1, guild_name = $2, interests = $3, skills = $4, 
            magic_abilities = $5, current_action = $6
        WHERE id = $7
        RETURNING id, name, guild_name, interests, skills, magic_abilities, 
                  position_x, position_y, position_z, current_action
        "#,
        name,
        guild_name,
        &interests as _,
        &skills as _,
        &magic_abilities as _,
        current_action,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(updated_character)
}

pub async fn update_location(
    pool: &Pool<Postgres>,
    id: Uuid,
    location: CharacterLocationDto,
) -> Result<(), AppError> {
    // First, check if the character exists
    let _ = get_by_id(pool, id).await?;

    // Update the location
    sqlx::query!(
        r#"
        UPDATE characters
        SET position_x = $1, position_y = $2, position_z = $3
        WHERE id = $4
        "#,
        location.x,
        location.y,
        location.z,
        id
    )
    .execute(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(())
} 