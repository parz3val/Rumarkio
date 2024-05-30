use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{store::supbase::SupaBase, types::database::WorkspaceStore};

pub async fn get_user_workspace(
    State(store): State<SupaBase>,
    Path(user_id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    match store.get_workspaces_by_user(user_id).await {
        Ok(workspaces) => return Ok(Json(workspaces)),
        Err(err) => {
            println!("Error : {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}
