use serde::{Deserialize, Serialize};
use welds::{errors::Result, WeldsModel};

use super::{project::Project, workspace_member::WorkspaceMember};

#[derive(Debug, Clone, Serialize, Deserialize, WeldsModel)]
#[welds(table = "d_workspaces")]
#[welds(db(Sqlite, Postgres))]
#[welds(BeforeCreate(before_create))]
#[welds(BeforeUpdate(before_update))]
#[welds(HasMany(projects, Project, "workspace_id"))]
#[welds(HasMany(members, WorkspaceMember, "workspace_id"))]
pub struct Workspace {
    #[welds(primary_key)]
    pub id: String,
    pub name: String,
    pub owner: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub active: bool,
}

pub async fn validate_workspace_table(client: &dyn welds::Client) -> anyhow::Result<Vec<welds::check::Issue>> {
    let issues = welds::check::schema::<Workspace>(client).await?;
    Ok(issues
        .into_iter()
        .filter(|i| i.level == welds::check::Level::Critical)
        .collect())
}

fn before_create(workspace: &mut Workspace) -> Result<()> {
    workspace.id = uuid::Uuid::new_v4().to_string();
    workspace.created_at = chrono::Utc::now().timestamp_millis();
    workspace.updated_at = chrono::Utc::now().timestamp_millis();
    Ok(())
}

fn before_update(workspace: &mut Workspace) -> Result<()> {
    workspace.updated_at = chrono::Utc::now().timestamp_millis();
    Ok(())
}
