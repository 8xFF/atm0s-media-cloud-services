use poem::{
    handler,
    web::{Data, Query},
    IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::{
        models::project::{ProjectCodecs, ProjectOptions},
        repositories::project::{get_projects, ProjectFilterDto},
    },
    http::HttpContext,
};

#[derive(Debug, Deserialize)]
pub struct SyncProjectQuery {
    pub secret: String,
}

#[derive(Debug, Serialize)]
pub struct SyncProjectData {
    pub app_id: String,
    pub app_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<String>,
    pub options: ProjectOptions,
    pub codecs: ProjectCodecs,
}

#[derive(Debug, Serialize)]
pub struct SyncProjectResponse {
    apps: Vec<SyncProjectData>,
}

/// Sync projects from database to other services
/// This validate with query secret
#[handler]
pub async fn sync_projects(query: Query<SyncProjectQuery>, data: Data<&HttpContext>) -> impl IntoResponse {
    async fn process(data: Data<&HttpContext>, query: SyncProjectQuery) -> anyhow::Result<SyncProjectResponse> {
        if query.secret != data.cfg.cluster_secret {
            return Err(anyhow::anyhow!("Invalid secret"));
        }

        let projects = get_projects(
            data.db.clone(),
            ProjectFilterDto {
                id: None,
                workspace_id: None,
                name: None,
            },
            None,
            None,
        )
        .await?;

        Ok(SyncProjectResponse {
            apps: projects
                .into_iter()
                .map(|p| {
                    let options: ProjectOptions = p
                        .options
                        .and_then(|o| serde_json::from_value(o).ok())
                        .unwrap_or_default();
                    let codecs: ProjectCodecs = p
                        .codecs
                        .and_then(|o| serde_json::from_value(o).ok())
                        .unwrap_or_default();
                    SyncProjectData {
                        app_id: p.id.clone(),
                        app_secret: p.secret.clone(),
                        hook: options.hook.clone(),
                        options,
                        codecs,
                    }
                })
                .collect(),
        })
    }

    http_common::response::to_response(process(data, query.0).await)
}
