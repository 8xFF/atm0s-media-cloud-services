use serde::{Deserialize, Serialize};
use welds::{errors::Result, WeldsModel};

#[derive(Debug, Clone, Serialize, Deserialize, WeldsModel)]
#[welds(table = "d_workspace_members")]
#[welds(db(Sqlite, Postgres))]
#[welds(BeforeCreate(before_create))]
#[welds(BeforeUpdate(before_update))]
#[welds(BelongsTo(workspace, super::workspace::Workspace, "workspace_id"))]
pub struct WorkspaceMember {
    #[welds(primary_key)]
    pub id: String,
    pub workspace_id: String,
    pub user_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}

pub async fn validate_workspace_members_table(client: &dyn welds::Client) -> anyhow::Result<Vec<welds::check::Issue>> {
    let issues = welds::check::schema::<WorkspaceMember>(client).await?;
    Ok(issues
        .into_iter()
        .filter(|i| i.level == welds::check::Level::Critical)
        .collect())
}

fn before_create(member: &mut WorkspaceMember) -> Result<()> {
    member.id = uuid::Uuid::new_v4().to_string();
    member.created_at = chrono::Utc::now().timestamp_millis();
    member.updated_at = chrono::Utc::now().timestamp_millis();
    Ok(())
}

fn before_update(member: &mut WorkspaceMember) -> Result<()> {
    member.updated_at = chrono::Utc::now().timestamp_millis();
    Ok(())
}
