use conundrum::ecosystem::db::db_traits::db_entity::DBEntity;

use crate::vector::models::joins::join_table::JoinTable;

pub struct GeneralJoin<L, R>
    where L: for<'a> DBEntity,
          R: for<'a> DBEntity {
    pub left: L,
    pub right: R,
    pub table: JoinTable,
}
