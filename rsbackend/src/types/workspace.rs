use serde::{Deserialize, Serialize};
use sqlx::prelude::{Decode, FromRow};
use validator::{Validate, ValidationError};

use super::user::User;

#[derive(FromRow, Deserialize, Serialize)]
pub struct Workspace {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub display_picture: Option<String>,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
    pub created_by: i64,
    pub verified: Option<bool>,
    pub total_members: Option<i64>,
}

impl TryFrom<&User> for Workspace {
    type Error = &'static str;
    fn try_from(user: &User) -> Result<Workspace, Self::Error> {
        return Ok(Workspace {
            id: None,
            name: format!("{:?}-workspace", user.username),
            description: format!("{:?}-workspace", user.username),
            display_picture: None,
            updated_at: None,
            created_at: None,
            verified: None,
            total_members: None,
            created_by: match user.id {
                Some(user_id) => user_id,
                None => return Err("No user id found. Can't created Workspace"),
            },
        });
    }
}
