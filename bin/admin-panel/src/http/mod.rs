mod api;
mod middleware;
mod view;

use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use clerk_rs::{
    clerk::Clerk,
    validators::{authorizer::ClerkAuthorizer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};
use http_common::response::to_response_error;
use poem::{
    listener::TcpListener,
    middleware::{AddData, Cors, Tracing},
    EndpointExt, Route, Server,
};
use view::build_frontend_route;

use crate::services::clerk_user::ClerkUserService;

#[derive(Clone)]
pub struct HttpCfg {
    pub cluster_secret: String,
    pub clerk_secret: String,
    pub clerk_publishable_key: String,
}

#[derive(Clone)]
pub struct HttpContext {
    pub db: Arc<dyn welds::Client>,
    pub http_client: reqwest::Client,
    pub clerk_user_service: ClerkUserService,
    pub clerk_authorizer: ClerkAuthorizer<MemoryCacheJwksProvider>,
    pub cfg: HttpCfg,
}

pub async fn run_http(port: u16, db: Arc<dyn welds::Client>, cfg: HttpCfg) -> anyhow::Result<()> {
    let clerk_config = ClerkConfiguration::new(None, None, Some(cfg.clerk_secret.to_string()), None);
    let clerk_client = Clerk::new(clerk_config);
    let provider: MemoryCacheJwksProvider = MemoryCacheJwksProvider::new(clerk_client.clone());
    let clerk_authorizer: ClerkAuthorizer<MemoryCacheJwksProvider> = ClerkAuthorizer::new(provider, true);

    let ctx = HttpContext {
        db,
        http_client: reqwest::Client::new(),
        clerk_user_service: ClerkUserService::new(clerk_client),
        clerk_authorizer,
        cfg: cfg.clone(),
    };

    let app = Route::new()
        .nest("/api", api::build_route(ctx.clone()))
        .nest("/", build_frontend_route())
        .with(AddData::new(ctx))
        .with(Cors::new())
        .with(Tracing)
        .catch_all_error(|e| async move { to_response_error(e.into()) });

    Server::new(TcpListener::bind(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port)))
        .name("admin-panel")
        .run(app)
        .await?;

    Ok(())
}
