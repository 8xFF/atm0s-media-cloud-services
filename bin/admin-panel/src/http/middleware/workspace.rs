use poem::{Endpoint, Error, IntoResponse, Middleware, Response, Result};
use reqwest::StatusCode;

use crate::{
    database::repositories::workspace::{get_workspace, WorkspaceFilterDto},
    http::HttpContext,
};

use super::clerk_auth::CLERK_USER_ID;

pub struct WorkspaceMiddleware {
    ctx: HttpContext,
}

impl WorkspaceMiddleware {
    pub fn new(ctx: HttpContext) -> Self {
        Self { ctx }
    }
}

impl<E: Endpoint> Middleware<E> for WorkspaceMiddleware {
    type Output = WorkspaceMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        WorkspaceMiddlewareImpl {
            endpoint: ep,
            ctx: self.ctx.clone(),
        }
    }
}

pub struct WorkspaceMiddlewareImpl<E> {
    endpoint: E,
    ctx: HttpContext,
}

#[derive(Debug, serde::Deserialize)]
struct WorkspacePath {
    workspace_id: String,
}

impl<E: Endpoint> Endpoint for WorkspaceMiddlewareImpl<E> {
    type Output = Response;

    async fn call(&self, req: poem::Request) -> Result<Self::Output> {
        // let mut req = req;
        let workspace_params = req.path_params::<WorkspacePath>()?;
        let workspace_id = workspace_params.workspace_id.clone();
        log::info!("workspace_id: {workspace_id}");

        let user_id = req
            .headers()
            .get(CLERK_USER_ID)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| poem::Error::from_string("missing user_id", StatusCode::BAD_REQUEST))?;

        match get_workspace(
            self.ctx.db.clone(),
            WorkspaceFilterDto {
                id: Some(workspace_id.clone()),
                user_id: Some(user_id.to_string()),
                ..Default::default()
            },
        )
        .await
        {
            Ok(workspace) => {
                if workspace.is_none() {
                    return Err(poem::Error::from_string("workspace not found", StatusCode::NOT_FOUND));
                }

                let res = self.endpoint.call(req).await;
                match res {
                    Ok(res) => Ok(res.into_response()),
                    Err(err) => {
                        log::error!("Error: {:?}", err);
                        Err(err)
                    }
                }
            }
            Err(err) => {
                log::error!("Error: {:?}", err);
                Err(Error::from_string(
                    "internal server error",
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        }
    }
}
