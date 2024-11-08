// USER HANDLERS

use crate::dto;
use crate::helpers;
use crate::models;
use crate::models::CustomTryFrom;
use crate::database;
use crate::AppState;
use axum::extract::{Json, Query, State};
use axum::response::IntoResponse;
use axum::Extension;
use uuid::Uuid;
use crate::dto::PaginationParams;

pub(crate) async fn sign_up(
    State(app_state): State<AppState>,
    Json(form): Json<dto::SignupForm>,
) -> Result<impl IntoResponse, helpers::ErrorType> {
    let model: models::UserModel = match form.try_into() {
        Ok(m) => m,
        Err(e) => {
            dbg!(e);
            return Err(helpers::api_error(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create user. Please try with another email or contact support"
                    .to_string(),
            ));
        }
    };
    let db = app_state.store.db;
    match database::create_user(&db, &model).await {
        Ok(()) => Ok(helpers::api_response("User created successfully")),
        Err(e) => {
            dbg!(e);
            Err(helpers::api_error(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create user. Please try with another email or contact support"
                    .to_string(),
            ))
        }
    }
}

pub(crate) async fn login(
    State(app_state): State<AppState>,
    Json(form): Json<dto::LoginForm>,
) -> Result<impl IntoResponse, helpers::ErrorType> {
    dbg!(form.clone());
    let db = app_state.store.db;
    let user = match database::get_user(&db, form.email.as_str()).await {
        Ok(model) => model,
        Err(e) => {
            dbg!(e);
            return Err(helpers::api_error(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch user".to_string(),
            ));
        }
    };
    let user_obj = user.clone();
    match helpers::verify_password(form.password, user.password_hash) {
        Ok(v) => {
            if !v {
                return Err(helpers::api_error(
                    http::StatusCode::UNAUTHORIZED,
                    "Password not match".to_string(),
                ));
            }
            println!("Password match");
        }
        Err(e) => {
            dbg!(e);
            return Err(helpers::api_error(
                http::StatusCode::UNAUTHORIZED,
                "Password not match".to_string(),
            ));
        }
    };
    let auth_user = dto::AuthUser {
        email: user_obj.email.clone(),
        id: user_obj.id,
    };
    let token = match helpers::create_auth_token(auth_user, "SECRET_KEY") {
        Ok(t) => t,
        Err(e) => {
            dbg!(e);
            return Err(helpers::api_error(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create auth token".to_string(),
            ));
        }
    };
    Ok(helpers::api_response(dto::LoginResponse {
        token,
        email: user_obj.email,
    }))
}

// --- LINKS --- //
pub async fn create_link(
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<dto::AuthUser>,
    Json(link): Json<dto::LinkForm>,
) -> Result<impl IntoResponse, helpers::ErrorType> {
    // let's check if session is provided
    let db = app_state.store.db;
    if let Ok(links_model) = models::LinkModel::c_try_from(link.clone(), auth_user.id) {
        if let Err(e) = database::create_link(&db, &links_model).await {
            dbg!(e);
            return Err(helpers::api_error(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create link".to_string(),
            ));
        }
        // Add data to tag, session, or collections if the
        // link is created successfully

        if let Some(session_id) = link.session_id {
            if let Err(e) = database::create_link_session(
                &db,
                &models::LinkSessionsModel {
                    id: Uuid::new_v4(),
                    links_id: links_model.id,
                    session_id,
                    created_at: None,
                    updated_at: None,
                },
            )
                .await
            {
                dbg!(e);
            }
        }
        if let Some(tag_id) = link.tag_id {
            if let Err(e) = database::create_link_tags(
                &db,
                &models::LinkTagsModel {
                    id: Uuid::new_v4(),
                    links_id: links_model.id,
                    tag_id,
                    created_at: None,
                    updated_at: None,
                },
            )
                .await
            {
                dbg!(e);
            }
        }
        if let Some(collections_id) = link.collection_id {
            if let Err(e) = database::create_link_collections(
                &db,
                &models::LinksCollectionsModel {
                    id: Uuid::new_v4(),
                    links_id: links_model.id,
                    collections_id,
                    created_at: None,
                    updated_at: None,
                },
            )
                .await
            {
                dbg!(e);
            }
        }

        let mdb = db.clone();
        if let Some(session_name) = link.session_name {
            println!("Session was provided so doing operations on session");
            tokio::spawn(async move {
                println!("Session Operations started");
                if let Ok(link_session) = database::get_user_session_by_title(&mdb, session_name.as_ref(), auth_user.id).await {
                    println!("Existing session with the name found");

                    let new_link_session = models::LinkSessionsModel {
                        id: Uuid::new_v4(),
                        links_id: links_model.id,
                        session_id: link_session.id,
                        created_at: None,
                        updated_at: None,
                    };

                    if let Err(e) = database::create_link_session(&mdb, &new_link_session).await {
                        println!("Failed to connect link to  the session");
                        dbg!(e);
                    };
                } else {
                    println!("Existing session with the name not found. Creating new session");
                    let new_session_model = models::SessionModel {
                        id: Uuid::new_v4(),
                        user_id: auth_user.id,
                        image_url: None,
                        title: session_name,
                        description: None,
                        reason: Some("daily-browse".to_string()),
                        created_at: None,
                        updated_at: None,
                    };

                    if let Ok(()) = database::create_session(&mdb, &new_session_model).await {
                        dbg!(&new_session_model);
                        println!("Session successfully created");
                        let link_sessions_model = models::LinkSessionsModel {
                            id: Uuid::new_v4(),
                            links_id: links_model.id,
                            session_id: new_session_model.id,
                            created_at: None,
                            updated_at: None,
                        };

                        if let Err(e) = database::create_link_session(&mdb, &link_sessions_model).await {
                            println!("Failed to connect link to  the session");
                            dbg!(e);
                        } else {
                            println!("Successfully connected the session and the link");
                        }
                    } else {
                        eprintln!("Failed to create session");
                    }
                }
            });
        }
        if let Some(tag_names) = link.tag_names {
            println!("Starting tag operations");
            tokio::spawn(async move {
                let tasks: Vec<_> = tag_names.iter().map(
                    |name| async {
                        if let Ok(link_tag) = database::get_tag_by_title(&db, name, auth_user.id).await {
                            println!("Tag with the name already exists");
                            // It means tag already exists so we'll just create a relationship
                            // between the tag and the link
                            let link_tag_model = models::LinkTagsModel {
                                id: Uuid::new_v4(),
                                links_id: links_model.id,
                                tag_id: link_tag.id,
                                created_at: None,
                                updated_at: None,
                            };

                            dbg!(&link_tag_model);

                            match database::create_link_tags(&db, &link_tag_model).await {
                                Ok(_) => {
                                    println!("Successfully connected tag with the link");
                                }
                                Err(e) => {
                                    eprintln!("Failed to connect tag to  the tag");
                                    dbg!(e);
                                }
                            };
                        } else {
                            println!("Tag with the name not found. Creating new tag");
                            // cause the given tag with the name doesn't exist,
                            // we'll have to create the new tag and assign it to our link links tag
                            let tag_model = models::TagModel {
                                id: Uuid::new_v4(),
                                title: name.to_string(),
                                description: None,
                                image_url: None,
                                user_id: auth_user.id,
                                created_at: None,
                                updated_at: None,
                            };

                            match database::create_tag(&db, &tag_model).await {
                                Ok(_) => {
                                    let link_tags_model = models::LinkTagsModel {
                                        id: Uuid::new_v4(),
                                        links_id: links_model.id,
                                        tag_id: tag_model.id,
                                        created_at: None,
                                        updated_at: None,
                                    };

                                    if let Err(e) = database::create_link_tags(&db, &link_tags_model).await {
                                        dbg!(e);
                                    };
                                }
                                Err(e) => {
                                    eprintln!("Failed to create new tag");
                                    dbg!(e);
                                }
                            }
                        }
                    }
                ).collect::<Vec<_>>();

                let _ = futures::future::join_all(tasks).await;


            });
        }
    }
    Ok(())
}

pub async fn get_links(
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<dto::AuthUser>,
    Query(pagination_params): Query<PaginationParams>,
) -> Result<impl IntoResponse, helpers::ErrorType> {
    // Let's see if we have any query params
    dbg!(pagination_params);
    let db = app_state.store.db;
    match database::get_links(&db, auth_user.id).await {
        Ok(d) => Ok(helpers::api_response(d)),
        Err(e) => Err(helpers::api_error(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}

// -- SESSION -- //

pub async fn create_session(
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<dto::AuthUser>,
    Json(form): Json<dto::SessionForm>,
) -> Result<impl IntoResponse, helpers::ErrorType> {
    let db = app_state.store.db;

    if let Ok(session_model) = models::SessionModel::c_try_from(form, auth_user.id) {
        if let Err(e) = database::create_session(&db, &session_model).await {
            dbg!(e);
            return Err(helpers::api_error(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create new session. Try again or contact support".to_string(),
            ));
        }
    }
    Ok(())
}

pub async fn get_sessions(
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<dto::AuthUser>,
) -> Result<impl IntoResponse, helpers::ErrorType> {
    let db = app_state.store.db;
    match database::get_sessions(&db, auth_user.id).await {
        Ok(d) => Ok(helpers::api_response(d)),
        Err(e) => Err(helpers::api_error(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}

// -- TAG -- //
pub async fn create_tag(
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<dto::AuthUser>,
    Json(form): Json<dto::TagForm>,
) -> Result<impl IntoResponse, helpers::ErrorType> {
    let db = app_state.store.db;

    if let Ok(tag_model) = models::TagModel::c_try_from(form, auth_user.id) {
        if let Err(e) = database::create_tag(&db, &tag_model).await {
            dbg!(e);
            return Err(helpers::api_error(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create new tag. Try again or contact support".to_string(),
            ));
        }
    }
    Ok(())
}

pub async fn get_tags(
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<dto::AuthUser>,
) -> Result<impl IntoResponse, helpers::ErrorType> {
    let db = app_state.store.db;
    match database::get_tags(&db, auth_user.id).await {
        Ok(d) => Ok(helpers::api_response(d)),
        Err(e) => Err(helpers::api_error(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}
// -- COLLECTION -- //

pub async fn create_collections(
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<dto::AuthUser>,
    Json(form): Json<dto::CollectionsForm>,
) -> Result<impl IntoResponse, helpers::ErrorType> {
    let db = app_state.store.db;

    if let Ok(collections_form) = models::CollectionsModel::c_try_from(form, auth_user.id) {
        if let Err(e) = database::create_collections(&db, &collections_form).await {
            dbg!(e);
            return Err(helpers::api_error(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create new collections. Try again or contact support".to_string(),
            ));
        }
    }
    Ok(())
}

pub async fn get_collections(
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<dto::AuthUser>,
) -> Result<impl IntoResponse, helpers::ErrorType> {
    let db = app_state.store.db;
    match database::get_collections(&db, auth_user.id).await {
        Ok(d) => Ok(helpers::api_response(d)),
        Err(e) => Err(helpers::api_error(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}
// -- FOLDER -- //

pub async fn create_folder(
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<dto::AuthUser>,
    Json(form): Json<dto::FolderForm>,
) -> Result<impl IntoResponse, helpers::ErrorType> {
    let db = app_state.store.db;

    if let Ok(folder_form) = models::FolderModel::c_try_from(form, auth_user.id) {
        if let Err(e) = database::create_folder(&db, &folder_form).await {
            dbg!(e);
            return Err(helpers::api_error(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create new folder. Try again or contact support".to_string(),
            ));
        }
    }
    Ok(())
}

pub async fn get_folders(
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<dto::AuthUser>,
) -> Result<impl IntoResponse, helpers::ErrorType> {
    let db = app_state.store.db;
    match database::get_folders(&db, auth_user.id).await {
        Ok(d) => Ok(helpers::api_response(d)),
        Err(e) => Err(helpers::api_error(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}
