use crate::types::database::NewDB;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub mod user;
pub mod workspace;

const SUPA_DB_URI: &str = "postgres://postgres.isgwozgajdzvwfbjbree:LAef2156cSglrfw@aws-0-ap-south-1.pooler.supabase.com:5432/postgres";

#[derive(Clone)]
pub struct SupaBase {
    db: Pool<Postgres>,
}

impl NewDB for SupaBase {
    async fn new() -> Self {
        let pool = match PgPoolOptions::new()
            .max_connections(3)
            .connect(SUPA_DB_URI)
            .await
        {
            Ok(pool) => pool,
            Err(e) => {
                println!("### : {:?}", e);
                panic!("Err");
            }
        };
        SupaBase { db: pool }
    }
}
