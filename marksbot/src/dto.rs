use serde::{Deserialize, Serialize};

// COMMON DTOS

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct PaginationParams {
    pub page: Option<usize>,
    pub max_count: Option<usize>,
}

// USERS DTOS
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SignupForm {
    pub email: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthUser {
    pub email: String,
    pub id: uuid::Uuid,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct LinkForm {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub session_id: Option<uuid::Uuid>,
    pub tag_id: Option<uuid::Uuid>,
    pub session_name: Option<String>,
    pub tag_names: Option<Vec<String>>,
    pub collection_id: Option<uuid::Uuid>,
    pub content: Option<String>,
    pub image_url: Option<String>,
}


// Sessions form and DTOs

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct SessionForm {
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub reason: Option<String>,
}

// -- Tag form and DTOS
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct TagForm {
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

// -- Collections form and DTOS
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct CollectionsForm {
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}


// -- Folder form and DTOS
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FolderForm {
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}
