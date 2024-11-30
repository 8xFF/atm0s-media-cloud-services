use poem::{
    handler,
    web::{Data, Json, Path},
    Error, IntoResponse,
};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    database::{
        models::project::{Project, ProjectCodecs, ProjectOptions},
        repositories::project::{
            count_projects, create_project, get_project, get_projects, update_project, CreateProjectDto,
            ProjectFilterDto, UpdateProjectDto,
        },
    },
    http::HttpContext,
};

#[derive(Debug, Deserialize)]
struct Params {
    workspace_id: String,
    project_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectBody {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectBody {
    pub name: Option<String>,
    pub options: Option<ProjectOptions>,
    pub codecs: Option<ProjectCodecs>,
}

#[handler]
pub async fn new_project(
    data: Data<&HttpContext>,
    body: Json<CreateProjectBody>,
    Path(workspace_id): Path<String>,
) -> impl IntoResponse {
    async fn process(
        data: Data<&HttpContext>,
        body: Json<CreateProjectBody>,
        workspace_id: String,
    ) -> anyhow::Result<Project> {
        let secret = utils::string::generate_api_key(32);
        let project = create_project(
            data.db.clone(),
            CreateProjectDto {
                name: body.name.clone(),
                secret,
                workspace_id: workspace_id.clone(),
                options: Some(Default::default()),
                codecs: Some(Default::default()),
            },
        )
        .await?;

        Ok(project)
    }

    http_common::response::to_response(process(data, body, workspace_id).await)
}

#[handler]
pub async fn list_projects(data: Data<&HttpContext>, Path(workspace_id): Path<String>) -> impl IntoResponse {
    async fn process(data: Data<&HttpContext>, workspace_id: String) -> anyhow::Result<(Vec<Project>, usize, usize)> {
        let filter = ProjectFilterDto {
            id: None,
            workspace_id: Some(workspace_id),
            name: None,
        };
        let projects = get_projects(data.db.clone(), filter.clone(), None, Some(0)).await?;
        let count = count_projects(data.db.clone(), filter.clone()).await?;

        Ok((projects, 0, count as usize))
    }

    http_common::response::to_response_list(process(data, workspace_id).await)
}

#[handler]
pub async fn project_detail(
    data: Data<&HttpContext>,
    Path(Params {
        workspace_id,
        project_id,
    }): Path<Params>,
) -> impl IntoResponse {
    async fn process(data: Data<&HttpContext>, workspace_id: String, project_id: String) -> anyhow::Result<Project> {
        match get_project(
            data.db.clone(),
            ProjectFilterDto {
                id: Some(project_id),
                workspace_id: Some(workspace_id),
                name: None,
            },
        )
        .await?
        {
            Some(project) => Ok(project),
            None => anyhow::bail!(Error::from_string("project not found", StatusCode::NOT_FOUND)),
        }
    }

    http_common::response::to_response(process(data, workspace_id, project_id).await)
}

#[handler]
pub async fn project_update(
    data: Data<&HttpContext>,
    Path(params): Path<Params>,
    body: Json<UpdateProjectBody>,
) -> impl IntoResponse {
    async fn process(
        data: Data<&HttpContext>,
        params: Params,
        body: Json<UpdateProjectBody>,
    ) -> anyhow::Result<Project> {
        update_project(
            data.db.clone(),
            ProjectFilterDto {
                id: Some(params.project_id),
                workspace_id: Some(params.workspace_id),
                name: None,
            },
            UpdateProjectDto {
                name: body.name.clone(),
                options: body.options.clone(),
                codecs: body.codecs.clone(),
            },
        )
        .await
    }

    http_common::response::to_response(process(data, params, body).await)
}
