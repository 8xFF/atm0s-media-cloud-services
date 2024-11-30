use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::database::models::project::{Project, ProjectCodecs, ProjectOptions};

#[derive(Debug, Clone, Default)]
pub struct ProjectFilterDto {
    pub id: Option<String>,
    pub workspace_id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectDto {
    pub name: String,
    pub secret: String,
    pub workspace_id: String,
    pub options: Option<ProjectOptions>,
    pub codecs: Option<ProjectCodecs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProjectDto {
    pub name: Option<String>,
    pub options: Option<ProjectOptions>,
    pub codecs: Option<ProjectCodecs>,
}

pub async fn create_project(client: Arc<dyn welds::Client>, dto: CreateProjectDto) -> anyhow::Result<Project> {
    let mut project = Project::new();
    project.name = dto.name.clone();
    project.workspace_id = dto.workspace_id.clone();
    project.secret = dto.secret.clone();
    project.options = dto.options.map(|o| serde_json::to_value(o).unwrap());
    project.codecs = dto.codecs.map(|o| serde_json::to_value(o).unwrap());

    project.save(client.as_ref()).await?;
    Ok(project.into_inner())
}

pub async fn get_projects(
    client: Arc<dyn welds::Client>,
    filter: ProjectFilterDto,
    limit: Option<u64>,
    offset: Option<u64>,
) -> anyhow::Result<Vec<Project>> {
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

pub async fn count_projects(client: Arc<dyn welds::Client>, filter: ProjectFilterDto) -> anyhow::Result<u64> {
    let query = build_query(filter);
    let count = query.count(client.as_ref()).await?;
    Ok(count)
}

pub async fn get_project(client: Arc<dyn welds::Client>, filter: ProjectFilterDto) -> anyhow::Result<Option<Project>> {
    let query = build_query(filter);
    let mut res = query.run(client.as_ref()).await?;
    match res.pop() {
        Some(project) => Ok(Some(project.into_inner())),
        None => Ok(None),
    }
}

pub async fn update_project(
    client: Arc<dyn welds::Client>,
    filter: ProjectFilterDto,
    dto: UpdateProjectDto,
) -> anyhow::Result<Project> {
    let query = build_query(filter);
    let mut res = query.run(client.as_ref()).await?;
    match res.pop() {
        Some(mut project) => {
            if let Some(name) = dto.name {
                project.name = name;
            }
            if let Some(opts) = dto.options {
                project.options = Some(serde_json::to_value(opts)?);
            }
            if let Some(codecs) = dto.codecs {
                project.codecs = Some(serde_json::to_value(codecs)?);
            }

            project.save(client.as_ref()).await?;
            Ok(project.into_inner())
        }
        None => anyhow::bail!("project not found"),
    }
}

pub async fn delete_project(client: Arc<dyn welds::Client>, id: String) -> anyhow::Result<bool> {
    match Project::find_by_id(client.as_ref(), id).await? {
        Some(mut prj) => {
            prj.delete(client.as_ref()).await?;
            Ok(true)
        }
        None => Ok(false),
    }
}

fn build_query(filter: ProjectFilterDto) -> welds::query::builder::QueryBuilder<Project> {
    let mut query = Project::all();
    if let Some(id) = filter.id {
        query = query.where_col(|c| c.id.equal(id.clone()));
    }

    if let Some(name) = filter.name {
        query = query.where_col(|c| c.name.equal(name.clone()));
    }
    if let Some(workspace_id) = filter.workspace_id {
        query = query.where_col(|c| c.workspace_id.equal(workspace_id.clone()));
    }

    query
}
