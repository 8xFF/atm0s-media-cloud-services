use std::fmt;

use serde::{Deserialize, Serialize};
use welds::{errors::Result, WeldsModel};

use super::project::Project;

#[derive(Debug, Clone, Deserialize)]
pub enum MemberRole {
    OWNER,
    ADMIN,
    MEMBER,
}

impl fmt::Display for MemberRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, WeldsModel, Serialize, Deserialize)]
#[welds(table = "d_project_members")]
#[welds(BelongsTo(project, Project, "project_id"))]
pub struct ProjectMember {
    #[welds(primary_key)]
    pub id: i32,

    pub project_id: String,

    pub user_id: String,

    pub role: String,
}

#[derive(Debug, Clone, WeldsModel, Serialize, Deserialize)]
#[welds(table = "t_project_invites")]
#[welds(BelongsTo(project, Project, "project_id"))]
#[welds(BeforeCreate(before_create_invite))]
pub struct ProjectInvite {
    #[welds(primary_key)]
    pub id: String,

    pub project_id: String,

    pub email: String,

    pub role: String,

    pub expire_at: i64,
}

fn before_create_invite(project_invite: &mut ProjectInvite) -> Result<()> {
    project_invite.id = uuid::Uuid::new_v4().to_string();
    Ok(())
}
