use welds::migrations::prelude::*;

use super::models::project::validate_project_table;

mod m20241102_init;

pub async fn check_tables(client: &dyn welds::Client) -> anyhow::Result<()> {
    let mut errors = vec![];
    errors.append(&mut validate_project_table(client).await?);

    if !errors.is_empty() {
        log::error!("Schema mismatch: found {:?} issues", errors);
        anyhow::bail!("Schema mismatch: found {} issues", errors.len());
    }

    Ok(())
}

pub async fn migration_up(client: &dyn welds::TransactStart) -> anyhow::Result<()> {
    up(client, &m20241102_init::up()).await?;
    Ok(())
}
