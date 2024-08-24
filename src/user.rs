use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Error, FromRow, PgPool};

use crate::handlers::UserInfo;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    name: String,
    verified: bool,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub async fn get_all(db_conn: &PgPool) -> Result<Vec<User>, Error> {
        query_as::<_, User>("SELECT * FROM users ORDER BY updated_at DESC")
            .fetch_all(db_conn)
            .await
    }

    pub async fn create(db_conn: &PgPool, user_info: UserInfo) -> Result<User, Error> {
        let now = chrono::Utc::now();
        let new_user = Self {
            name: user_info.given_name,
            verified: user_info.verified_email,
            email: user_info.email,
            created_at: now,
            updated_at: now,
        };
        let result = query(
            r#"
          INSERT INTO users (name, verified, email, created_at, updated_at)
          VALUES($1, $2, $3, $4, $5)
          "#,
        )
        .bind(&new_user.name)
        .bind(new_user.verified)
        .bind(&new_user.email)
        .bind(new_user.created_at)
        .bind(new_user.updated_at)
        .execute(db_conn)
        .await;

        match result {
            Ok(_) => Ok(new_user),
            Err(err) => Err(err),
        }
    }
}
