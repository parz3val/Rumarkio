use super::SupaBase;
use crate::types::{
    database::UserStore,
    user::{User, UserProfileData},
};
use sqlx::{postgres::PgRow, Row};

impl UserStore for SupaBase {
    async fn create_user(&self, user: User) -> Result<User, sqlx::error::Error> {
        match sqlx::query(
            r#"INSERT INTO USERS (email, username, password) VALUES ($1, $2, $3) RETURNING id"#,
        )
        .bind(user.email.clone())
        .bind(user.username.clone())
        .bind(user.password.clone())
        .fetch_one(&self.db)
        .await
        {
            Ok(row) => {
                let id = row.get(0);
                return Ok(User {
                    id: Some(id),
                    username: user.username,
                    email: user.email,
                    password: user.password,
                    created_at: user.updated_at,
                    updated_at: user.created_at,
                    verified: user.verified,
                });
            }
            Err(e) => return Err(e),
        }
    }

    async fn get_user_by_email(&self, email: String) -> Result<User, sqlx::error::Error> {
        match sqlx::query_as::<_, User>(r#"SELECT * FROM USERS where id = $1"#)
            .bind(email)
            .fetch_one(&self.db)
            .await
        {
            Ok(row) => return Ok(row),
            Err(e) => return Err(e),
        }
    }

    async fn get_user_profile_by_id(&self, id: i64) -> Result<UserProfileData, sqlx::error::Error> {
        match sqlx::query_as::<_, UserProfileData>(
            r#"
                    SELECT 
                    USERS.ID,
                    USERS.CREATED_AT,
                    USERS.UPDATED_AT,
                    USERS.USERNAME,
                    USERS.EMAIL,
                    USERS.VERIFIED,
                    MEMBER_PROFILE.FIRST_NAME,
                    MEMBER_PROFILE.LAST_NAME,
                    MEMBER_PROFILE.COUNTRY_CODE,
                    MEMBER_PROFILE.COUNTRY,
                    MEMBER_PROFILE.CITY,
                    MEMBER_PROFILE.STREET_ADDRESS,
                    MEMBER_PROFILE.PROFILE_PICTURE,
                    MEMBER_PROFILE.USER_ID
                    FROM USERS LEFT JOIN MEMBER_PROFILE ON USERS.ID = MEMBER_PROFILE.USER_ID
                    WHERE
                    USERS.ID = $1
        "#,
        )
        .bind(id)
        .fetch_one(&self.db)
        .await
        {
            Ok(row_) => return Ok(row_),
            Err(e) => return Err(e),
        }
    }
}
