use conundrum::ecosystem::db::tables::DatabaseTable;

use crate::vector::{database::db_traits::db_entity::DBEntity, models::joins::join_table::JoinTable};

pub struct GeneralJoin<L: DBEntity, R: DBEntity> {
    pub left: L,
    pub right: R,
    pub table: JoinTable,
}
