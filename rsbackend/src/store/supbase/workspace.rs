use crate::types::{database::WorkspaceStore, workspace::Workspace};
use sqlx::{postgres::PgRow, Row};

use super::SupaBase;

impl WorkspaceStore for SupaBase {
    async fn create_workspace(
        &self,
        workspace: Workspace,
    ) -> Result<Workspace, sqlx::error::Error> {
        match sqlx::query(r#"INSERT INTO WORKSPACES (name, description, display_picture, created_by) VALUES ($1, $2, $3, $4) RETURNING id"#)
            .bind(workspace.name.clone())
            .bind(workspace.description.clone())
            .bind(workspace.display_picture.clone())
            .bind(workspace.created_by)
            .fetch_one(&self.db)
            .await
        {
            Ok(row) => {
                let id: i64 = row.get(0);
                return Ok(Workspace{
                            name: workspace.name,
                            id: id,
                            description: workspace.description,
                            created_by: workspace.created_by,
                            display_picture: None,
                            updated_at: None,
                            created_at: None,
                            verified: None,
                            total_members: None,
                })
            },
            Err(e) => return Err(e),
        }
    }

    async fn get_workspaces_by_user(
        &self,
        created_by: i64,
    ) -> Result<Vec<Workspace>, sqlx::error::Error> {
        match sqlx::query_as::<_, Workspace>(r#""#)
            .bind(created_by)
            .fetch_all(&self.db)
            .await
        {
            Ok(row) => return Ok(row),
            Err(e) => return Err(e),
        }
    }
}
