use axum::http::{
    header::{ACCEPT, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use axum::middleware;
use axum::routing::{get, post};
use tower_http::cors::{Any, CorsLayer};

// const DB_PASS: &str = "HarryDBPassAwesome1234";
const DB_URI: &str = "postgresql://postgres.nwxhmrihcazeqwuldqyk:HarryDBPassAwesome1234@aws-0-ap-south-1.pooler.supabase.com:5432/postgres";
pub(crate) mod dto;
pub(crate) mod handlers;
pub(crate) mod helpers;
pub(crate) mod middlewares;
pub(crate) mod models;
pub(crate) mod services;
pub(crate) mod sqlite_db;
pub(crate) mod database;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) store: database::PostgresStore,
}

pub(crate) async fn my_router() -> axum::Router {
    let state = AppState {
        store: database::PostgresStore::new(DB_URI).await,
    };
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            AUTHORIZATION,
            ACCEPT,
            CONTENT_TYPE,
            ACCESS_CONTROL_ALLOW_ORIGIN,
        ])
        .allow_origin(Any);
    axum::Router::new()
        .route("/", get(|| async { "hello rumarkio" }))
        .route("/api/v1/users/create", post(handlers::sign_up))
        .route("/api/v1/users/login", post(handlers::login))
        .route(
            "/api/v1/links/create",
            post(handlers::create_link)
                .layer(middleware::from_fn(middlewares::authorization_middleware)),
        )
        .route(
            "/api/v1/links",
            get(handlers::get_links)
                .layer(middleware::from_fn(middlewares::authorization_middleware)),
        )
        .route(
            "/api/v1/sessions/create",
            post(handlers::create_session)
                .layer(middleware::from_fn(middlewares::authorization_middleware)),
        )
        .route(
            "/api/v1/sessions",
            get(handlers::get_sessions)
                .layer(middleware::from_fn(middlewares::authorization_middleware)),
        )
        .route(
            "/api/v1/tags/create",
            post(handlers::create_tag)
                .layer(middleware::from_fn(middlewares::authorization_middleware)),
        )
        .route(
            "/api/v1/tags",
            get(handlers::get_tags)
                .layer(middleware::from_fn(middlewares::authorization_middleware)),
        )
        .route(
            "/api/v1/collections/create",
            post(handlers::create_collections)
                .layer(middleware::from_fn(middlewares::authorization_middleware)),
        )
        .route(
            "/api/v1/collections",
            get(handlers::get_collections)
                .layer(middleware::from_fn(middlewares::authorization_middleware)),
        )
        .route(
            "/api/v1/folders/create",
            post(handlers::create_folder)
                .layer(middleware::from_fn(middlewares::authorization_middleware)),
        )
        .route(
            "/api/v1/folders",
            get(handlers::get_folders)
                .layer(middleware::from_fn(middlewares::authorization_middleware)),
        )
        .layer(cors)
        .with_state(state)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = my_router().await;

    let server = async {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
        tracing::debug!("Listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();
    };
    server.await;
}
