use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{Decode, FromRow};
use validator::{Validate, ValidationError};

use crate::helpers::auth::encrypt_password;

use super::workspace::{self, Workspace};

#[derive(FromRow, Decode, Deserialize, Serialize)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
    pub verified: Option<bool>,
}

impl TryFrom<SignupForm> for User {
    type Error = &'static str;

    fn try_from(form: SignupForm) -> Result<User, Self::Error> {
        match form.validate() {
            Ok(_) => (),
            Err(_) => return Err("Validation Error"),
        }
        match encrypt_password(form.password) {
            Ok(encrypted_password) => Ok(User {
                id: None,
                username: form.username,
                email: form.email,
                password: encrypted_password,
                created_at: None,
                updated_at: None,
                verified: Some(false),
            }),
            Err(_) => Err("Error occured while doing password encryption"),
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct SignupForm {
    #[validate(email)]
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignedupUser {
    pub id: u64,
    pub email: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignupResponse {
    data: SignedupUser,
}

pub struct UserProfile {
    pub user_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country_code: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub street_address: Option<String>,
    pub workspace: i64,
}

impl UserProfile {
    pub fn new(workspace: &Workspace, user: &User) -> Result<Self, core::fmt::Error> {
        match workspace.id {
            Some(id) => {
                return Ok(Self {
                    user_id: user.id,
                    workspace: id,
                    first_name: None,
                    last_name: None,
                    country_code: None,
                    country: None,
                    city: None,
                    street_address: None,
                })
            }
            None => return Err(core::fmt::Error),
        }
    }
}

#[derive(FromRow, Deserialize, Serialize)]
pub struct UserProfileData {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
    pub verified: Option<bool>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country_code: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub street_address: Option<String>,
}
