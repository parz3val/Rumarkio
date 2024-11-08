use crate::dto;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// USER MODELS
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub(crate) struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl TryFrom<dto::SignupForm> for UserModel {
    type Error = &'static str;
    fn try_from(value: dto::SignupForm) -> Result<Self, Self::Error> {
        let id = Uuid::new_v4();
        match bcrypt::hash(value.password, bcrypt::DEFAULT_COST) {
            Ok(v) => Ok(Self {
                id,
                email: value.email,
                password_hash: v,
                created_at: None,
                updated_at: None,
            }),
            Err(_) => Err("Failed to verify user form"),
        }
    }
}

// LINKS MODELS
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub(crate) struct LinkModel {
    pub id: Uuid,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub content: Option<String>,
    pub user_id: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
pub trait CustomTryFrom<T> {
    type Error;
    fn c_try_from(value: T, user_id: Uuid) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl CustomTryFrom<dto::LinkForm> for LinkModel {
    type Error = &'static str;
    fn c_try_from(value: dto::LinkForm, user_id: Uuid) -> Result<Self, Self::Error> {
        let id = Uuid::new_v4();
        Ok(Self {
            id,
            url: value.url,
            title: value.title,
            description: value.description,
            image_url: value.image_url,
            content: value.content,
            user_id,
            created_at: None,
            updated_at: None,
        })
    }
}

// Session Models
#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct SessionModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub image_url: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub reason: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl CustomTryFrom<dto::SessionForm> for SessionModel {
    type Error = &'static str;

    fn c_try_from(value: dto::SessionForm, user_id: Uuid) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            image_url: value.image_url,
            title: value.title,
            description: value.description,
            reason: value.reason,
            created_at: None,
            updated_at: None,
        })
    }
}

// -- TAGS
#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct TagModel {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub user_id: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl CustomTryFrom<dto::TagForm> for TagModel {
    type Error = &'static str;
    fn c_try_from(value: dto::TagForm, user_id: Uuid) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {
            id: Uuid::new_v4(),
            title: value.title,
            description: value.description,
            image_url: value.image_url,
            user_id,
            created_at: None,
            updated_at: None,
        })
    }
}

// -- COLLECTIONS
#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct CollectionsModel {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub user_id: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl CustomTryFrom<dto::CollectionsForm> for CollectionsModel {
    type Error = &'static str;
    fn c_try_from(value: dto::CollectionsForm, user_id: Uuid) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {
            id: Uuid::new_v4(),
            title: value.title,
            description: value.description,
            image_url: value.image_url,
            user_id,
            created_at: None,
            updated_at: None,
        })
    }
}
// -- Folder
#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct FolderModel {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub user_id: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl CustomTryFrom<dto::FolderForm> for FolderModel {
    type Error = &'static str;
    fn c_try_from(value: dto::FolderForm, user_id: Uuid) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {
            id: Uuid::new_v4(),
            title: value.title,
            description: value.description,
            image_url: value.image_url,
            user_id,
            created_at: None,
            updated_at: None,
        })
    }
}

// LINK SESSIONS
#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct LinkSessionsModel {
    pub id: Uuid,
    pub links_id: Uuid,
    pub session_id: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

// LINK TAGS

#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
pub struct LinkTagsModel {
    pub id: Uuid,
    pub links_id: Uuid,
    pub tag_id: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct TagsCollectionsModel {
    pub id: Uuid,
    pub tag_id: Uuid,
    pub collections_id: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct LinksCollectionsModel {
    pub id: Uuid,
    pub links_id: Uuid,
    pub collections_id: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
