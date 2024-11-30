use poem::{
    handler,
    web::{Data, Path},
    IntoResponse,
};

use crate::{
    database::{
        models::workspace_member::WorkspaceMember,
        repositories::workspace_member::{count_workspace_members, get_workspace_members, WorkspaceMemberFilter},
    },
    http::HttpContext,
};

#[handler]
pub async fn list_members(data: Data<&HttpContext>, Path(workspace_id): Path<String>) -> impl IntoResponse {
    async fn process(
        data: Data<&HttpContext>,
        workspace_id: String,
    ) -> anyhow::Result<(Vec<WorkspaceMember>, usize, usize)> {
        let filter = WorkspaceMemberFilter {
            workspace_id: Some(workspace_id.clone()),
            ..Default::default()
        };
        let members = get_workspace_members(data.db.clone(), filter.clone(), None, None).await?;
        let count = count_workspace_members(data.db.clone(), filter).await?;

        Ok((members, 0, count as usize))
    }

    http_common::response::to_response_list(process(data, workspace_id).await)
}
