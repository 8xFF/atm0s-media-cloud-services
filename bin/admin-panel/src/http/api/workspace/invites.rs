use http_common::response::StatusResponse;
use poem::{
    handler,
    web::{Data, Json, Path},
    Error, IntoResponse,
};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    database::{
        models::workspace_invite::WorkspaceMembersInvite,
        repositories::{
            workspace_invite::{
                count_workspace_member_invites, create_workspace_member_invite, delete_many_workspace_invite,
                delete_workspace_invite, get_workspace_member_invite, get_workspace_member_invites,
                update_workspace_member_invite_by_id, WorkspaceMembersInviteFilter, WorkspaceMembersInviteUpdateDto,
            },
            workspace_member::{
                create_workspace_member, get_workspace_member, WorkspaceMemberCreate, WorkspaceMemberFilter,
            },
        },
    },
    http::{middleware::clerk_auth::ClerkUserId, HttpContext},
};

#[derive(Debug, Deserialize)]
pub struct InivteBody {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub workspace_id: String,
    pub invite_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct DeleteInviteBody {
    pub ids: Vec<i32>,
}

#[handler]
pub async fn invite(
    data: Data<&HttpContext>,
    body: Json<InivteBody>,
    Path(workspace_id): Path<String>,
) -> impl IntoResponse {
    async fn process(
        data: Data<&HttpContext>,
        workspace_id: String,
        body: Json<InivteBody>,
    ) -> anyhow::Result<WorkspaceMembersInvite> {
        let user = data.clerk_user_service.find_user_by_email(&body.email).await?;
        if let Some(user) = user {
            if let Some(user_id) = user.id.clone() {
                let member = get_workspace_member(
                    data.db.clone(),
                    WorkspaceMemberFilter {
                        workspace_id: Some(workspace_id.clone()),
                        user_id: Some(user_id.clone()),
                        ..Default::default()
                    },
                )
                .await?;

                if member.is_some() {
                    anyhow::bail!("User is already a member of this workspace");
                }
            }
        }

        let now = chrono::Utc::now().timestamp_millis();
        let expires = now + 1000 * 60 * 60 * 24 * 2; // 2 days

        match get_workspace_member_invite(
            data.db.clone(),
            WorkspaceMembersInviteFilter {
                workspace_id: Some(workspace_id.clone()),
                email: Some(body.email.clone()),
                ..Default::default()
            },
        )
        .await
        {
            Ok(Some(invite)) => {
                let invite = update_workspace_member_invite_by_id(
                    data.db.clone(),
                    invite.id,
                    WorkspaceMembersInviteUpdateDto { expires: Some(expires) },
                )
                .await?;
                Ok(invite)
            }
            Ok(None) => {
                let invite = create_workspace_member_invite(
                    data.db.clone(),
                    crate::database::repositories::workspace_invite::WorkspaceMembersInviteCreate {
                        workspace_id,
                        email: body.email.clone(),
                        expires,
                    },
                )
                .await?;
                Ok(invite)
            }
            Err(e) => anyhow::bail!(e),
        }
    }

    http_common::response::to_response(process(data, workspace_id, body).await)
}

#[handler]
pub async fn accept_invite(
    data: Data<&HttpContext>,
    Path(params): Path<Params>,
    user_id: ClerkUserId,
) -> impl IntoResponse {
    async fn process(data: Data<&HttpContext>, params: Params, user_id: String) -> anyhow::Result<StatusResponse> {
        let email = data.clerk_user_service.get_user_email(&user_id).await?;
        let invt = get_workspace_member_invite(
            data.db.clone(),
            WorkspaceMembersInviteFilter {
                workspace_id: Some(params.workspace_id.clone()),
                email: Some(email.clone()),
                id: Some(params.invite_id),
            },
        )
        .await?;

        if invt.is_none() {
            anyhow::bail!(Error::from_string(
                "invite not found".to_string(),
                StatusCode::NOT_FOUND
            ));
        }

        let invt = invt.unwrap();
        if invt.expires < chrono::Utc::now().timestamp_millis() {
            anyhow::bail!(Error::from_string(
                "invite expired".to_string(),
                StatusCode::BAD_REQUEST
            ));
        }

        let _ = create_workspace_member(
            data.db.clone(),
            WorkspaceMemberCreate {
                workspace_id: params.workspace_id.clone(),
                user_id: user_id.clone(),
            },
        )
        .await?;

        let _ = delete_workspace_invite(data.db.clone(), invt.id).await?;

        Ok(StatusResponse { status: true })
    }

    http_common::response::to_response(process(data, params, user_id.into()).await)
}

#[handler]
pub async fn decline_invite(
    data: Data<&HttpContext>,
    Path(params): Path<Params>,
    user_id: ClerkUserId,
) -> impl IntoResponse {
    async fn process(data: Data<&HttpContext>, params: Params, user_id: String) -> anyhow::Result<StatusResponse> {
        let email = data.clerk_user_service.get_user_email(&user_id).await?;
        let invt = get_workspace_member_invite(
            data.db.clone(),
            WorkspaceMembersInviteFilter {
                workspace_id: Some(params.workspace_id.clone()),
                email: Some(email.clone()),
                id: Some(params.invite_id),
            },
        )
        .await?;

        let status = if invt.is_some() {
            delete_workspace_invite(data.db.clone(), invt.unwrap().id).await?
        } else {
            true
        };

        Ok(StatusResponse { status })
    }

    http_common::response::to_response(process(data, params, user_id.into()).await)
}

#[handler]
pub async fn list_my_invites(
    data: Data<&HttpContext>,
    Path(workspace_id): Path<String>,
    user_id: ClerkUserId,
) -> impl IntoResponse {
    async fn process(
        data: Data<&HttpContext>,
        workspace_id: String,
        user_id: String,
    ) -> anyhow::Result<(Vec<WorkspaceMembersInvite>, usize, usize)> {
        let email = data.clerk_user_service.get_user_email(&user_id).await?;
        let invites = get_workspace_member_invites(
            data.db.clone(),
            WorkspaceMembersInviteFilter {
                workspace_id: Some(workspace_id.clone()),
                email: Some(email.clone()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

        let count = count_workspace_member_invites(
            data.db.clone(),
            WorkspaceMembersInviteFilter {
                workspace_id: Some(workspace_id.clone()),
                email: Some(email.clone()),
                ..Default::default()
            },
        )
        .await?;

        Ok((invites, 0, count as usize))
    }

    http_common::response::to_response_list(process(data, workspace_id, user_id.into()).await)
}

#[handler]
pub async fn list_invites(data: Data<&HttpContext>, Path(workspace_id): Path<String>) -> impl IntoResponse {
    async fn process(
        data: Data<&HttpContext>,
        workspace_id: String,
    ) -> anyhow::Result<(Vec<WorkspaceMembersInvite>, usize, usize)> {
        let invites = get_workspace_member_invites(
            data.db.clone(),
            WorkspaceMembersInviteFilter {
                workspace_id: Some(workspace_id.clone()),
                ..Default::default()
            },
            None,
            None,
        )
        .await?;

        let count = count_workspace_member_invites(
            data.db.clone(),
            WorkspaceMembersInviteFilter {
                workspace_id: Some(workspace_id.clone()),
                ..Default::default()
            },
        )
        .await?;

        Ok((invites, 0, count as usize))
    }

    http_common::response::to_response_list(process(data, workspace_id).await)
}

#[handler]
pub async fn delete_invite(
    data: Data<&HttpContext>,
    Path(workspace_id): Path<String>,
    body: Json<DeleteInviteBody>,
) -> impl IntoResponse {
    async fn process(data: Data<&HttpContext>, workspace_id: String, ids: Vec<i32>) -> anyhow::Result<StatusResponse> {
        let status = delete_many_workspace_invite(data.db.clone(), workspace_id, ids).await?;
        Ok(StatusResponse { status })
    }

    http_common::response::to_response(process(data, workspace_id, body.ids.clone()).await)
}
