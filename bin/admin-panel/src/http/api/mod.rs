mod configs;
mod sync;
mod users;
mod workspace;

use poem::{get, handler, EndpointExt, IntoResponse, Route};
use serde::Serialize;

use super::{
    middleware::{self},
    HttpContext,
};

#[derive(Debug, Serialize)]
pub struct HealthCheckResponse {
    pub status: bool,
    pub version: String,
}

#[handler]
pub async fn health_check() -> impl IntoResponse {
    http_common::response::to_response(Ok(HealthCheckResponse {
        status: true,
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}

pub fn build_route(ctx: HttpContext) -> Route {
    Route::new()
        .at("/health", get(health_check))
        .at("/sync/projects", get(sync::sync_projects))
        .nest(
            "/workspaces",
            workspace::build_route(ctx.clone()).with(middleware::clerk_auth::ClerkAuthMiddleware::new(ctx.clone())),
        )
        .nest("/configs", configs::build_route())
        .nest(
            "/users",
            users::build_route().with(middleware::clerk_auth::ClerkAuthMiddleware::new(ctx.clone())),
        )
}
