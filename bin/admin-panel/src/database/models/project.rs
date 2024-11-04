use serde::{Deserialize, Serialize};
use welds::{errors::Result, WeldsModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectOptions {
    pub create_automatically: Option<bool>,
    pub admin_mute: Option<bool>,
    pub record: Option<bool>,
}

impl Default for ProjectOptions {
    fn default() -> Self {
        Self {
            create_automatically: Some(true),
            admin_mute: Some(false),
            record: Some(false),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCodecs {
    pub h264: Option<bool>,
    pub vp9: Option<bool>,
    pub opus: Option<bool>,
    pub aac: Option<bool>,
}

impl Default for ProjectCodecs {
    fn default() -> Self {
        Self {
            h264: Some(true),
            vp9: Some(false),
            opus: Some(true),
            aac: Some(false),
        }
    }
}

#[derive(Debug, Clone, WeldsModel, Serialize, Deserialize)]
#[welds(table = "d_projects")]
#[welds(db(Sqlite, Postgres))]
#[welds(BeforeCreate(before_create))]
#[welds(BeforeUpdate(before_update))]
pub struct Project {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub secret: String,
    pub options: Option<serde_json::Value>,
    pub codecs: Option<serde_json::Value>,
}

fn before_create(project: &mut Project) -> Result<()> {
    project.id = uuid::Uuid::new_v4().to_string();
    Ok(())
}

fn before_update(_project: &mut Project) -> Result<()> {
    Ok(())
}
