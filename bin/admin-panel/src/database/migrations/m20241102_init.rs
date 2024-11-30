use welds::errors::Result;
use welds::migrations::prelude::*;

pub fn up() -> Vec<MigrationFn> {
    vec![
        m20241102_create_projects,
        m20241125_create_workspace,
        m20241125_create_workspace_members,
        m20241125_create_workspace_members_invite,
    ]
}

pub fn m20241102_create_projects(_: &TableState) -> Result<MigrationStep> {
    let m = create_table("d_projects")
        .id(|c| c("id", Type::String))
        .column(|c| c("workspace_id", Type::String).create_index())
        .column(|c| c("name", Type::String))
        .column(|c| c("secret", Type::String).create_unique_index())
        .column(|c| c("options", Type::Json))
        .column(|c| c("codecs", Type::Json))
        .column(|c| c("created_at", Type::IntBig))
        .column(|c| c("updated_at", Type::IntBig));
    Ok(MigrationStep::new("create projects", m))
}

pub fn m20241125_create_workspace(_: &TableState) -> Result<MigrationStep> {
    let m = create_table("d_workspaces")
        .id(|c| c("id", Type::String))
        .column(|c| c("name", Type::String))
        .column(|c| c("owner", Type::String))
        .column(|c| c("created_at", Type::IntBig))
        .column(|c| c("updated_at", Type::IntBig))
        .column(|c| c("active", Type::Bool));
    Ok(MigrationStep::new("create workspaces", m))
}

pub fn m20241125_create_workspace_members(_: &TableState) -> Result<MigrationStep> {
    let m = create_table("d_workspace_members")
        .id(|c| c("id", Type::String))
        .column(|c| c("workspace_id", Type::String).create_index())
        .column(|c| c("user_id", Type::String).create_index())
        .column(|c| c("created_at", Type::IntBig))
        .column(|c| c("updated_at", Type::IntBig));

    Ok(MigrationStep::new("create workspaces member", m))
}

pub fn m20241125_create_workspace_members_invite(_: &TableState) -> Result<MigrationStep> {
    let m = create_table("t_workspace_members_invite")
        .id(|c| c("id", Type::Int))
        .column(|c| c("workspace_id", Type::String).create_index())
        .column(|c| c("email", Type::String))
        .column(|c| c("expires", Type::IntBig))
        .column(|c| c("created_at", Type::IntBig))
        .column(|c| c("updated_at", Type::IntBig));

    Ok(MigrationStep::new("create workspaces member invites", m))
}
