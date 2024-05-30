use axum::extract::Path;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Error;

use crate::types::database::WorkspaceStore;
use crate::types::user::UserProfile;
use crate::types::workspace::Workspace;
use crate::{
    store::supbase::SupaBase,
    types::{
        database::UserStore,
        user::{SignupForm, User},
    },
};

pub async fn create_user(
    State(store): State<SupaBase>,
    Json(form): Json<SignupForm>,
) -> &'static str {
    let user = match User::try_from(form) {
        Ok(user) => user,
        Err(err) => return err,
    };
    match store.create_user(user).await {
        Ok(user) => {
            let workspace = match Workspace::try_from(user) {
                Ok(workspace) => workspace,
                Err(e) => return "Failed to create workspace",
            };
            match store.create_workspace(workspace).await {
                Ok(workspace) => {
                    let user_profile = UserProfile::new(&workspace, &user);
                    return "User Created";
                }
                Err(_e) => return "",
            };
        }
        Err(_e) => return "Error creating user: {}",
    }
}

pub async fn get_user_profile(
    State(store): State<SupaBase>,
    Path(user_id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    match store.get_user_profile_by_id(user_id).await {
        Ok(user_data) => return Ok(Json(user_data)),
        Err(err) => {
            println!("Error {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}
