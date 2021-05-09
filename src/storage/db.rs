use anyhow::Result;
use sqlx::PgPool;

#[derive(serde::Deserialize, sqlx::FromRow)]
pub(crate) struct Pokemon {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub shakespeare_description: Option<String>,
}

pub(crate) async fn get_pokemon(pool: &PgPool, pokemon_name: &str) -> Result<Pokemon> {
    let result: Pokemon = sqlx::query_as(
        "SELECT id, name, description, shakespeare_description FROM pokemon where name = $1",
    )
    .bind(pokemon_name)
    .fetch_one(pool)
    .await?;
    Ok(result)
}

pub(crate) async fn store_pokemon(
    pool: &PgPool,
    id: i32,
    name: &str,
    description: &str,
    shakespeare_description: &str,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO pokemon (id, name, description, shakespeare_description) VALUES($1, $2, $3, $4)",
    )
    .bind(id)
    .bind(name)
    .bind(description)
    .bind(shakespeare_description)
    .execute(pool)
    .await?;
    Ok(())
}

#[allow(dead_code)]
async fn delete_rows_pokemon(pool: &PgPool) -> Result<()> {
    sqlx::query("DELETE FROM pokemon").execute(pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[actix_rt::test]
    async fn test_store_and_read() {
        let settings =
            crate::configuration::get_configuration().expect("Failed to read configuration.");
        let connection_pool = PgPool::connect(&settings.database.connection_string())
            .await
            .expect("Failed to connect to Postgres.");

        let drop = delete_rows_pokemon(&connection_pool).await;
        assert!(drop.is_ok());

        let empty_table_result = get_pokemon(&connection_pool, "pikachu").await;
        assert!(empty_table_result.is_err());

        let store_result = store_pokemon(&connection_pool, 25, "pikachu", "When several of these POKéMON gather, their electricity could build and cause lightning storms.", "At which hour several of these pokémon gather,  their electricity couldst buildeth and cause lightning storms.").await;
        assert!(store_result.is_ok());

        let query_result = get_pokemon(&connection_pool, "pikachu").await;
        assert!(query_result.is_ok());

        let result = query_result.unwrap();
        assert_eq!(result.id, 25);
        assert_eq!(result.description.unwrap(), "When several of these POKéMON gather, their electricity could build and cause lightning storms.");
        assert_eq!(result.shakespeare_description.unwrap(), "At which hour several of these pokémon gather,  their electricity couldst buildeth and cause lightning storms.");

        let drop = delete_rows_pokemon(&connection_pool).await;
        assert!(drop.is_ok());
    }
}
