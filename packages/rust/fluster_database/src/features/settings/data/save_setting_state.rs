use crate::core::database::db::get_database;
use fluster_core_utilities::core_types::fluster_error::FlusterResult;

use super::settings_entity::SettingsEntity;

pub async fn save_setting_state(json_string: String, settings_id: String) -> FlusterResult<()> {
    let db_res = get_database().await;
    let db = db_res.lock().await;
    SettingsEntity::save(&db, json_string, settings_id).await?;
    Ok(())
}
