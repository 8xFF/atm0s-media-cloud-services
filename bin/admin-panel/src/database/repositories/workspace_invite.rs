use std::sync::Arc;

use crate::database::models::workspace_invite::WorkspaceMembersInvite;

#[derive(Debug, Clone)]
pub struct WorkspaceMembersInviteCreate {
    pub workspace_id: String,
    pub email: String,
    pub expires: i64,
}

#[derive(Debug, Clone, Default)]
pub struct WorkspaceMembersInviteUpdateDto {
    pub expires: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct WorkspaceMembersInviteFilter {
    pub id: Option<i32>,
    pub workspace_id: Option<String>,
    pub email: Option<String>,
}

pub async fn create_workspace_member_invite(
    client: Arc<dyn welds::Client>,
    dto: WorkspaceMembersInviteCreate,
) -> anyhow::Result<WorkspaceMembersInvite> {
    let mut invite = WorkspaceMembersInvite::new();
    invite.workspace_id = dto.workspace_id.clone();
    invite.email = dto.email.clone();
    invite.expires = dto.expires;
    invite.save(client.as_ref()).await?;
    Ok(invite.into_inner())
}

pub async fn update_workspace_member_invite_by_id(
    client: Arc<dyn welds::Client>,
    id: i32,
    dto: WorkspaceMembersInviteUpdateDto,
) -> anyhow::Result<WorkspaceMembersInvite> {
    match WorkspaceMembersInvite::find_by_id(client.as_ref(), id).await? {
        Some(mut invite) => {
            if let Some(expires) = dto.expires {
                invite.expires = expires;
            }
            invite.save(client.as_ref()).await?;
            Ok(invite.into_inner())
        }
        None => anyhow::bail!("Invite not found"),
    }
}

pub async fn get_workspace_member_invites(
    client: Arc<dyn welds::Client>,
    filter: WorkspaceMembersInviteFilter,
    limit: Option<u64>,
    offset: Option<u64>,
) -> anyhow::Result<Vec<WorkspaceMembersInvite>> {
    let mut query = build_query(filter);
    if let Some(limit) = limit {
        query = query.limit(limit as i64);
    }
    if let Some(offset) = offset {
        query = query.offset(offset as i64);
    }
    let res = query.run(client.as_ref()).await?;
    Ok(res.into_iter().map(|p| p.into_inner()).collect())
}

pub async fn get_workspace_member_invite(
    client: Arc<dyn welds::Client>,
    filter: WorkspaceMembersInviteFilter,
) -> anyhow::Result<Option<WorkspaceMembersInvite>> {
    let query = build_query(filter);
    let mut res = query.run(client.as_ref()).await?;
    match res.pop() {
        Some(invite) => Ok(Some(invite.into_inner())),
        None => Ok(None),
    }
}

pub async fn count_workspace_member_invites(
    client: Arc<dyn welds::Client>,
    filter: WorkspaceMembersInviteFilter,
) -> anyhow::Result<u64> {
    let query = build_query(filter);
    let count = query.count(client.as_ref()).await?;
    Ok(count)
}

pub async fn delete_workspace_invite(client: Arc<dyn welds::Client>, id: i32) -> anyhow::Result<bool> {
    match WorkspaceMembersInvite::find_by_id(client.as_ref(), id).await? {
        Some(mut invite) => {
            invite.delete(client.as_ref()).await?;
            Ok(true)
        }
        None => Ok(false),
    }
}

pub async fn delete_many_workspace_invite(
    client: Arc<dyn welds::Client>,
    workspace_id: String,
    ids: Vec<i32>,
) -> anyhow::Result<bool> {
    for id in ids {
        let filter = WorkspaceMembersInviteFilter {
            id: Some(id),
            workspace_id: Some(workspace_id.clone()),
            ..Default::default()
        };
        let query = build_query(filter);
        query.delete(client.as_ref()).await?;
    }
    Ok(true)
}

fn build_query(filter: WorkspaceMembersInviteFilter) -> welds::query::builder::QueryBuilder<WorkspaceMembersInvite> {
    let mut query = WorkspaceMembersInvite::all();
    if let Some(id) = filter.id {
        query = query.where_col(|c| c.id.equal(id));
    }
    if let Some(workspace_id) = filter.workspace_id {
        query = query.where_col(|c| c.workspace_id.equal(workspace_id.clone()));
    }
    if let Some(email) = filter.email {
        query = query.where_col(|c| c.email.equal(email.clone()));
    }
    query
}
