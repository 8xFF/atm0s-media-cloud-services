mod projects;
mod users;

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
        .nest("/health", get(health_check))
        .nest(
            "/users",
            users::build_route().with(middleware::clerk_auth::ClerkAuthMiddleware::new(ctx.clone())),
        )
        .nest("/projects", projects::build_route(ctx.clone()))
}
