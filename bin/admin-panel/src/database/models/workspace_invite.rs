use serde::{Deserialize, Serialize};
use welds::{errors::Result, WeldsModel};

use super::workspace::Workspace;

#[derive(Debug, Clone, Serialize, Deserialize, WeldsModel)]
#[welds(table = "t_workspace_members_invite")]
#[welds(db(Sqlite, Postgres))]
#[welds(BeforeCreate(before_create))]
#[welds(BeforeUpdate(before_update))]
#[welds(BelongsTo(workspace, Workspace, "workspace_id"))]
pub struct WorkspaceMembersInvite {
    #[welds(primary_key)]
    pub id: i32,
    pub workspace_id: String,
    pub email: String,
    pub expires: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

pub async fn validate_workspace_members_invite_table(
    client: &dyn welds::Client,
) -> anyhow::Result<Vec<welds::check::Issue>> {
    let issues = welds::check::schema::<WorkspaceMembersInvite>(client).await?;
    Ok(issues
        .into_iter()
        .filter(|i| i.level == welds::check::Level::Critical)
        .collect())
}

fn before_create(workspace: &mut WorkspaceMembersInvite) -> Result<()> {
    workspace.created_at = chrono::Utc::now().timestamp_millis();
    workspace.updated_at = chrono::Utc::now().timestamp_millis();
    Ok(())
}

fn before_update(workspace: &mut WorkspaceMembersInvite) -> Result<()> {
    workspace.updated_at = chrono::Utc::now().timestamp_millis();
    Ok(())
}
