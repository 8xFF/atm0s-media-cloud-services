use clerk_rs::validators::authorizer::{ClerkError, ClerkRequest};
use poem::{
    http::HeaderValue, Endpoint, Error, FromRequest, IntoResponse, Middleware, Request, RequestBody, Response, Result,
};
use reqwest::StatusCode;

use crate::http::HttpContext;

pub(crate) const CLERK_USER_ID: &str = "x-clerk-user-id";

pub struct ClerkUserId(String);

impl ClerkUserId {
    pub fn into(self) -> String {
        self.0
    }
}

impl<'a> FromRequest<'a> for ClerkUserId {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let user_id = req
            .headers()
            .get(CLERK_USER_ID)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| Error::from_string("missing user_id", StatusCode::BAD_REQUEST))?;
        Ok(ClerkUserId(user_id.to_string()))
    }
}

pub struct ClerkAuthMiddleware {
    ctx: HttpContext,
}

pub struct RequestWrapper<'a> {
    pub req: &'a Request,
}

impl ClerkRequest for RequestWrapper<'_> {
    fn get_header(&self, key: &str) -> Option<String> {
        self.req
            .headers()
            .get(key)
            .map(|header| header.to_str().expect("header must be a string").to_string())
    }

    fn get_cookie(&self, key: &str) -> Option<String> {
        self.req.cookie().get(key).map(|cookie| cookie.value_str().to_string())
    }
}

impl ClerkAuthMiddleware {
    pub fn new(ctx: HttpContext) -> Self {
        Self { ctx }
    }
}

impl<E: Endpoint> Middleware<E> for ClerkAuthMiddleware {
    type Output = ClerkAuthMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        ClerkAuthMiddlewareImpl {
            endpoint: ep,
            ctx: self.ctx.clone(),
        }
    }
}

pub struct ClerkAuthMiddlewareImpl<E> {
    endpoint: E,
    ctx: HttpContext,
}

impl<E: Endpoint> Endpoint for ClerkAuthMiddlewareImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let mut req = req;
        let wrapper = RequestWrapper { req: &req };
        let authorizer = self.ctx.clerk_authorizer.clone();

        match authorizer.authorize(&wrapper).await {
            Ok(jwt) => {
                let headers = req.headers_mut();
                headers.insert(CLERK_USER_ID, HeaderValue::from_str(&jwt.sub).unwrap());
                let res = self.endpoint.call(req).await;
                match res {
                    Ok(resp) => {
                        let resp = resp.into_response();
                        Ok(resp)
                    }
                    Err(err) => {
                        log::error!("[ClerkAuthMiddlewareImpl] call endpoint error: {err}");
                        Err(err)
                    }
                }
            }
            Err(e) => match e {
                ClerkError::Unauthorized(msg) => Err(Error::from_string(msg, StatusCode::UNAUTHORIZED)),
                ClerkError::InternalServerError(msg) => Err(Error::from_string(msg, StatusCode::INTERNAL_SERVER_ERROR)),
            },
        }
    }
}
