use axum::routing::post;
use axum::{routing::get, Router};

use crate::handlers::user::{create_user, get_user_profile};
use crate::handlers::workspace::{self, get_user_workspace};
use crate::types::database::NewDB;
use crate::{handlers::root, store::supbase::SupaBase};
use tower_http::trace::TraceLayer;

fn user_router(store: SupaBase) -> Router {
    return Router::new()
        .route("/register", post(create_user))
        .route("/profile/:id", get(get_user_profile))
        .with_state(store);
}

fn workspace_router(store: SupaBase) -> Router {
    return Router::new()
        .route("/all/:id", get(get_user_workspace))
        .with_state(store);
}

pub async fn web_router() -> Router {
    let store = SupaBase::new().await;
    let user_router = user_router(store.clone());
    let workspace_router = workspace_router(store.clone());
    let app = Router::new()
        .route("/", get(root))
        .nest("/api/v1/users", user_router)
        .nest("/api/v1/workspaces", workspace_router)
        .layer(TraceLayer::new_for_http());
    return app;
}
