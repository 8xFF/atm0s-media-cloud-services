use poem::{get, handler, web::Data, Error, IntoResponse, Route};
use reqwest::StatusCode;

use crate::http::{middleware::clerk_auth::ClerkUserId, HttpContext};

#[handler]
pub async fn get_me(data: Data<&HttpContext>, user_id: ClerkUserId) -> impl IntoResponse {
    async fn process(data: Data<&HttpContext>, user_id: String) -> anyhow::Result<clerk_rs::models::User> {
        match data.clerk_user_service.get_user_by_id(&user_id).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => anyhow::bail!(Error::from_string("user not found".to_string(), StatusCode::NOT_FOUND)),
            Err(e) => anyhow::bail!(e),
        }
    }

    http_common::response::to_response(process(data, user_id.into()).await)
}

pub fn build_route() -> Route {
    Route::new().nest("/me", get(get_me))
}
