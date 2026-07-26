use conundrum::ecosystem::db::tables::DatabaseTable;

use crate::vector::database::{db_types::db_entity::DBEntity, joins::join_table::JoinTable};

pub struct GeneralJoin<L = DatabaseTable, R = DatabaseTable> {
    pub left: L,
    pub right: R,
    pub table: JoinTable,
}
