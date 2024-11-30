use serde::{Deserialize, Serialize};
use welds::{errors::Result, WeldsModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,
}

impl Default for ProjectOptions {
    fn default() -> Self {
        Self {
            hook: None,
            record: Some(false),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCodecs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h264: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vp9: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vp8: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opus: Option<bool>,
}

impl Default for ProjectCodecs {
    fn default() -> Self {
        Self {
            h264: Some(true),
            vp9: Some(true),
            vp8: Some(true),
            opus: Some(true),
        }
    }
}

#[derive(Debug, Clone, WeldsModel, Serialize, Deserialize)]
#[welds(table = "d_projects")]
#[welds(db(Sqlite, Postgres))]
#[welds(BeforeCreate(before_create))]
#[welds(BeforeUpdate(before_update))]
pub struct Project {
    #[welds(primary_key)]
    pub id: String,
    pub name: String,
    pub workspace_id: String,
    pub secret: String,
    pub options: Option<serde_json::Value>,
    pub codecs: Option<serde_json::Value>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub async fn validate_project_table(client: &dyn welds::Client) -> anyhow::Result<Vec<welds::check::Issue>> {
    let issues = welds::check::schema::<Project>(client).await?;
    Ok(issues
        .into_iter()
        .filter(|i| i.level == welds::check::Level::Critical)
        .collect())
}

fn before_create(project: &mut Project) -> Result<()> {
    project.id = uuid::Uuid::new_v4().to_string();
    project.created_at = chrono::Utc::now().timestamp_millis();
    project.updated_at = chrono::Utc::now().timestamp_millis();
    Ok(())
}

fn before_update(project: &mut Project) -> Result<()> {
    project.updated_at = chrono::Utc::now().timestamp_millis();
    Ok(())
}
