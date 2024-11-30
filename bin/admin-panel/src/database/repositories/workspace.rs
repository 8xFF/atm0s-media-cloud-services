use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::database::models::{workspace::Workspace, workspace_member::WorkspaceMember};

#[derive(Debug, Clone, Default)]
pub struct WorkspaceFilterDto {
    pub id: Option<String>,
    pub name: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkspaceDto {
    pub name: String,
    pub owner: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkspaceDto {
    pub name: Option<String>,
}

pub async fn create_workspace(client: Arc<dyn welds::Client>, dto: CreateWorkspaceDto) -> anyhow::Result<Workspace> {
    let mut workspace = Workspace::new();
    workspace.name = dto.name.clone();
    workspace.owner = dto.owner.clone();

    workspace.save(client.as_ref()).await?;
    Ok(workspace.into_inner())
}

pub async fn get_workspaces(
    client: Arc<dyn welds::Client>,
    filter: WorkspaceFilterDto,
    limit: Option<u64>,
    offset: Option<u64>,
) -> anyhow::Result<Vec<Workspace>> {
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

pub async fn count_workspaces(client: Arc<dyn welds::Client>, filter: WorkspaceFilterDto) -> anyhow::Result<u64> {
    let query = build_query(filter);
    let count = query.count(client.as_ref()).await?;
    Ok(count)
}

pub async fn update_workspace(
    client: Arc<dyn welds::Client>,
    id: String,
    dto: UpdateWorkspaceDto,
) -> anyhow::Result<Workspace> {
    match Workspace::find_by_id(client.as_ref(), id).await? {
        Some(mut w) => {
            if let Some(name) = dto.name {
                w.name = name.clone();
            }
            w.save(client.as_ref()).await?;
            Ok(w.into_inner())
        }
        None => Err(anyhow::anyhow!("Workspace not found")),
    }
}

pub async fn get_workspace(
    client: Arc<dyn welds::Client>,
    filter: WorkspaceFilterDto,
) -> anyhow::Result<Option<Workspace>> {
    let query = build_query(filter);
    let mut res = query.run(client.as_ref()).await?;
    match res.pop() {
        Some(workspace) => Ok(Some(workspace.into_inner())),
        None => Ok(None),
    }
}

pub async fn delete_workspace(client: Arc<dyn welds::Client>, id: String) -> anyhow::Result<bool> {
    match Workspace::find_by_id(client.as_ref(), id).await? {
        Some(mut workspace) => {
            workspace.delete(client.as_ref()).await?;
            Ok(true)
        }
        None => Ok(false),
    }
}

fn build_query(filter: WorkspaceFilterDto) -> welds::query::builder::QueryBuilder<Workspace> {
    let mut query = Workspace::all();
    if let Some(id) = filter.id {
        query = query.where_col(|c| c.id.equal(id.clone()));
    }
    if let Some(name) = filter.name {
        query = query.where_col(|c| c.name.equal(name.clone()));
    }

    if let Some(user_id) = filter.user_id {
        let member_query = WorkspaceMember::where_col(|c| c.user_id.equal(user_id.clone()));
        query = query.where_relation(|w| w.members, member_query);
    }
    query
}
