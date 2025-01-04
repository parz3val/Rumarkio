use core::panic;

// Supabase URL
// const DB_URI: &str = "postgresql://postgres.nwxhmrihcazeqwuldqyk:HarryDBPassAwesome1234@aws-0-ap-south-1.pooler.supabase.com:5432/postgres";
// Local database URL
const DB_URI: &str = "postgresql://postgres:rumarkio@localhost:5432/rumarkio_test";
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let pool = match sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URI)
        .await
    {
        Ok(db) => db,
        Err(e) => panic!("Failed to connect to db because of :: => {}", e),
    };
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("Successfully migrated"),
        Err(e) => panic!("Failed to migrate because of :: => {}", e),
    };
}
