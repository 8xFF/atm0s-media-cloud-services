use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::database::models::project::{Project, ProjectCodecs, ProjectOptions};

#[derive(Debug, Clone)]
pub struct ProjectFilterDto {
    pub owner: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectDto {
    pub name: String,
    pub owner: String,
    pub secret: String,
    pub options: Option<ProjectOptions>,
    pub codecs: Option<ProjectCodecs>,
}

pub async fn create_project(client: Arc<dyn welds::Client>, dto: CreateProjectDto) -> anyhow::Result<Project> {
    let mut project = Project::new();
    project.name = dto.name.clone();
    project.owner = dto.owner.clone();
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

fn build_query(filter: ProjectFilterDto) -> welds::query::builder::QueryBuilder<Project> {
    let mut query = Project::all();
    if let Some(owner) = filter.owner {
        query = query.where_col(|c| c.owner.equal(owner.clone()));
    }
    if let Some(name) = filter.name {
        query = query.where_col(|c| c.name.equal(name.clone()));
    }
    query
}
