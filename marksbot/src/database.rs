use crate::models;
use sqlx::{ postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;
// DEFINITIONS FOR STORE

// INITIALIZE DATABASE
#[derive(Clone)]
pub(crate) struct PostgresStore {
    pub(crate) db: Pool<Postgres>,
}

impl PostgresStore {
    pub(crate) async fn new(db_uri: &str) -> Self {
        let db = match PgPoolOptions::new()
            .max_connections(1)
            .connect(db_uri)
            .await
        {
            Ok(db) => db,
            Err(e) => {
                panic!("Failed to connect to db because of :: => {}", e);
            }
        };
        Self { db }
    }
}

// USERS
pub(crate) async fn create_user(
    db: &Pool<Postgres>,
    model: &models::UserModel,
) -> Result<(), sqlx::Error> {
    if let Err(e) = sqlx::query(
        r#"
    insert into users (id, email, password_hash)
    values ($1, $2, $3)
    "#,
    )
        .bind(&model.id)
        .bind(&model.email)
        .bind(&model.password_hash)
        .execute(db)
        .await
    {
        return Err(e);
    }
    Ok(())
}

pub(crate) async fn get_user(
    db: &Pool<Postgres>,
    email: &str,
) -> Result<models::UserModel, sqlx::Error> {
    return Ok(
        sqlx::query_as::<_, models::UserModel>(r#" select * from users where email = $1"#)
            .bind(email)
            .fetch_one(db)
            .await?,
    );
}

// LINKS
pub(crate) async fn create_link(
    db: &Pool<Postgres>,
    model: &models::LinkModel,
) -> Result<(), sqlx::Error> {
    if let Err(e) = sqlx::query(
        r#"
        insert into user_links (id, url, title, description, image_url, content, user_id)
        values ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
        .bind(&model.id)
        .bind(&model.url)
        .bind(&model.title)
        .bind(&model.description)
        .bind(&model.image_url)
        .bind(&model.content)
        .bind(&model.user_id)
        .execute(db)
        .await
    {
        return Err(e);
    };
    Ok(())
}

pub(crate) async fn get_links(
    db: &Pool<Postgres>,
    reference: uuid::Uuid,
) -> Result<Vec<models::LinkModel>, sqlx::Error> {
    Ok(sqlx::query_as::<_, models::LinkModel>(
        r#"
        select * from user_links where user_id = $1;
        "#,
    )
        .bind(reference)
        .fetch_all(db)
        .await?)
}

// SESSION
pub(crate) async fn create_session(
    db: &Pool<Postgres>,
    model: &models::SessionModel,
) -> Result<(), sqlx::Error> {
    if let Err(e) = sqlx::query(
        r#"
        insert into user_sessions
        (
        id,
        user_id,
        image_url,
        title,
        description,
        reason
        )
        values
        ($1, $2, $3, $4, $5, $6)
        "#,
    )
        .bind(&model.id)
        .bind(&model.user_id)
        .bind(&model.image_url)
        .bind(&model.title)
        .bind(&model.description)
        .bind(&model.reason)
        .execute(db)
        .await
    {
        return Err(e);
    }
    Ok(())
}

pub(crate) async fn get_sessions(
    db: &Pool<Postgres>,
    reference: uuid::Uuid,
) -> Result<Vec<models::SessionModel>, sqlx::Error> {
    Ok(sqlx::query_as::<_, models::SessionModel>(
        r#"
        select * from user_sessions where user_id = $1;
        "#,
    )
        .bind(reference)
        .fetch_all(db)
        .await?)
}

pub(crate) async fn get_user_session_by_title(
    db: &Pool<Postgres>,
    session_title: &str,
    user_id: Uuid
) -> Result<models::SessionModel, sqlx::Error>
{
    Ok(sqlx::query_as::<_, models::SessionModel>(r#"
    select * from user_sessions where title = $1 and user_id = $2;
    "#)
        .bind(session_title)
        .bind(user_id)
        .fetch_one(db)
        .await?
    )
}
// -- TAG
pub async fn create_tag(db: &Pool<Postgres>, model: &models::TagModel) -> Result<(), sqlx::Error> {
    if let Err(e) = sqlx::query(
        r#"
        insert into user_tags(id,user_id,image_url,title,description)
        values
        ($1, $2, $3, $4, $5);
        "#,
    )
        .bind(&model.id)
        .bind(&model.user_id)
        .bind(&model.image_url)
        .bind(&model.title)
        .bind(&model.description)
        .execute(db)
        .await
    {
        return Err(e);
    }
    Ok(())
}

pub(crate) async fn get_tags(
    db: &Pool<Postgres>,
    reference: uuid::Uuid,
) -> Result<Vec<models::TagModel>, sqlx::Error> {
    Ok(sqlx::query_as::<_, models::TagModel>(
        r#"
        select * from user_tags where user_id = $1;
        "#,
    )
        .bind(reference)
        .fetch_all(db)
        .await?)
}

pub(crate) async fn _get_tags_by_title(
    db: &Pool<Postgres>,
    tag_titles: Vec<String>,
    user_id: Uuid
) -> Result<Vec<models::TagModel>, sqlx::Error>
{
    Ok(sqlx::query_as::<_, models::TagModel>(
        r#"
        select * from user_tags where user_id = $1 and title = ANY($2);
        "#,
    )
        .bind(user_id)
        .bind(tag_titles)
        .fetch_all(db)
        .await?)
}
pub(crate) async fn get_tag_by_title(
    db: &Pool<Postgres>,
    tag_title: &str,
    user_id: Uuid
) -> Result<models::TagModel, sqlx::Error>
{
    Ok(sqlx::query_as::<_, models::TagModel>(
        r#"
        select * from user_tags where user_id = $1 and title =$2;
        "#,
    )
        .bind(user_id)
        .bind(tag_title)
        .fetch_one(db)
        .await?)
}

// -- COLLECTIONS
pub async fn create_collections(
    db: &Pool<Postgres>,
    model: &models::CollectionsModel,
) -> Result<(), sqlx::Error> {
    if let Err(e) = sqlx::query(
        r#"
        insert into user_collections(id, user_id, image_url, title, description )
        values
        ($1, $2, $3, $4, $5)
        "#,
    )
        .bind(&model.id)
        .bind(&model.user_id)
        .bind(&model.image_url)
        .bind(&model.title)
        .bind(&model.description)
        .execute(db)
        .await
    {
        return Err(e);
    }
    Ok(())
}

pub(crate) async fn get_collections(
    db: &Pool<Postgres>,
    reference: uuid::Uuid,
) -> Result<Vec<models::CollectionsModel>, sqlx::Error> {
    Ok(sqlx::query_as::<_, models::CollectionsModel>(
        r#"
        select * from user_collections where user_id = $1;
        "#,
    )
        .bind(reference)
        .fetch_all(db)
        .await?)
}

pub async fn create_folder(
    db: &Pool<Postgres>,
    model: &models::FolderModel,
) -> Result<(), sqlx::Error> {
    if let Err(e) = sqlx::query(
        r#"
        insert into user_folders
        ( id, user_id, image_url, title, description)
        values
        ($1, $2, $3, $4, $5)
        "#,
    )
        .bind(&model.id)
        .bind(&model.user_id)
        .bind(&model.image_url)
        .bind(&model.title)
        .bind(&model.description)
        .execute(db)
        .await
    {
        return Err(e);
    }
    Ok(())
}

pub(crate) async fn get_folders(
    db: &Pool<Postgres>,
    reference: uuid::Uuid,
) -> Result<Vec<models::FolderModel>, sqlx::Error> {
    Ok(sqlx::query_as::<_, models::FolderModel>(
        r#"
        select * from user_folders where user_id = $1;
        "#,
    )
        .bind(reference)
        .fetch_all(db)
        .await?)
}

// -- LINK SESSIONS
pub(crate) async fn create_link_session(
    db: &Pool<Postgres>,
    model: &models::LinkSessionsModel,
) -> Result<(), sqlx::Error> {
    if let Err(e) =
        sqlx::query("insert into link_sessions (id, link_id, session_id) values ($1, $2, $3);")
            .bind(&model.id)
            .bind(&model.links_id)
            .bind(&model.session_id)
            .execute(db)
            .await
    {
        return Err(e);
    }
    Ok(())
}

// LINK TAGS

pub(crate) async fn create_link_tags(
    db: &Pool<Postgres>,
    model: &models::LinkTagsModel,
) -> Result<(), sqlx::Error> {
    if let Err(e) = sqlx::query(
        "insert into links_tags (id,links_id,tag_id) values ($1, $2, $3);"
    )
        .bind(&model.id)
        .bind(&model.links_id)
        .bind(&model.tag_id)
        .execute(db)
        .await
    {
        return Err(e);
    }
    Ok(())
}

// LINK COLLECTIONS
pub(crate) async fn create_link_collections(
    db: &Pool<Postgres>,
    model: &models::LinksCollectionsModel,
) -> Result<(), sqlx::Error> {
    if let Err(e) = sqlx::query(
        "insert into links_tags (id,links_id,collections_id) values ($1, $2, $3);",
    )
        .bind(&model.id)
        .bind(&model.links_id)
        .bind(&model.collections_id)
        .execute(db)
        .await
    {
        return Err(e);
    }
    Ok(())
}
