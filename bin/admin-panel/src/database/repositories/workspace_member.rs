use std::sync::Arc;

use crate::database::models::workspace_member::WorkspaceMember;

#[derive(Debug, Clone)]
pub struct WorkspaceMemberCreate {
    pub workspace_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Default)]
pub struct WorkspaceMemberFilter {
    pub id: Option<String>,
    pub workspace_id: Option<String>,
    pub user_id: Option<String>,
}

pub async fn create_workspace_member(
    client: Arc<dyn welds::Client>,
    dto: WorkspaceMemberCreate,
) -> anyhow::Result<WorkspaceMember> {
    let mut member = WorkspaceMember::new();
    member.workspace_id = dto.workspace_id.clone();
    member.user_id = dto.user_id.clone();
    member.save(client.as_ref()).await?;
    Ok(member.into_inner())
}

pub async fn get_workspace_members(
    client: Arc<dyn welds::Client>,
    filter: WorkspaceMemberFilter,
    limit: Option<u64>,
    offset: Option<u64>,
) -> anyhow::Result<Vec<WorkspaceMember>> {
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

pub async fn get_workspace_member(
    client: Arc<dyn welds::Client>,
    filter: WorkspaceMemberFilter,
) -> anyhow::Result<Option<WorkspaceMember>> {
    let query = build_query(filter);
    let mut res = query.run(client.as_ref()).await?;
    match res.pop() {
        Some(member) => Ok(Some(member.into_inner())),
        None => Ok(None),
    }
}

pub async fn count_workspace_members(
    client: Arc<dyn welds::Client>,
    filter: WorkspaceMemberFilter,
) -> anyhow::Result<u64> {
    let query = build_query(filter);
    let res = query.count(client.as_ref()).await?;
    Ok(res)
}

pub async fn delete_workspace_members(
    client: Arc<dyn welds::Client>,
    workspace_id: String,
    ids: Vec<String>,
) -> anyhow::Result<bool> {
    for id in ids {
        let filter = WorkspaceMemberFilter {
            id: Some(id),
            workspace_id: Some(workspace_id.clone()),
            ..Default::default()
        };
        let query = build_query(filter);
        query.delete(client.as_ref()).await?;
    }
    Ok(true)
}

fn build_query(filter: WorkspaceMemberFilter) -> welds::query::builder::QueryBuilder<WorkspaceMember> {
    let mut query = WorkspaceMember::all();
    if let Some(id) = filter.id {
        query = query.where_col(|c| c.id.equal(id.clone()));
    }
    if let Some(workspace_id) = filter.workspace_id {
        query = query.where_col(|c| c.workspace_id.equal(workspace_id.clone()));
    }
    if let Some(user_id) = filter.user_id {
        query = query.where_col(|c| c.user_id.equal(user_id.clone()));
    }
    query
}
