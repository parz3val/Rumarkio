use std::future::Future;

use sqlx::error::Error;

use super::{
    user::{User, UserProfile, UserProfileData},
    workspace::Workspace,
};

pub trait NewDB {
    fn new() -> impl Future<Output = Self>
    where
        Self: Sized;
}

pub trait WorkspaceStore {
    fn create_workspace(
        &self,
        workspace: Workspace,
    ) -> impl Future<Output = Result<Workspace, Error>>;
    fn get_workspaces_by_user(
        &self,
        created_by: i64,
    ) -> impl Future<Output = Result<Vec<Workspace>, Error>>;
}

pub trait UserStore {
    fn create_user(&self, user: User) -> impl Future<Output = Result<User, Error>>;
    fn get_user_by_email(&self, email: String) -> impl Future<Output = Result<User, Error>>;
    fn get_user_profile_by_id(
        &self,
        id: i64,
    ) -> impl Future<Output = Result<UserProfileData, Error>>;
    fn create_user_profile(
        &self,
        profile: UserProfile,
    ) -> impl Future<Output = Result<UserProfile, Error>>;
}
pub trait LibraryStore {}
pub trait BookmarkStore {}
pub trait CollectionStore {}
pub trait TagStore {}
pub trait SessionStore {}
