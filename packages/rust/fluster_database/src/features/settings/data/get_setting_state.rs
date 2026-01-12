use crate::core::database::db::get_database;
use fluster_core_utilities::core_types::fluster_error::FlusterResult;

use super::settings_entity::SettingsEntity;

// WARNING: I removed the settingId prop on 6-22-25 because it wasn't being used. If issues arise,
// add it back in and added it as part of an `only_if` clause in the query.

pub async fn get_setting_state() -> FlusterResult<String> {
    let db_res = get_database().await;
    let db = db_res.lock().await;
    let res = SettingsEntity::get_settings(&db).await?;
    Ok(res)
}
