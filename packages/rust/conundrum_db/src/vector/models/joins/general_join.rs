use conundrum::ecosystem::db::db_traits::db_entity::DBEntity;

use crate::vector::models::joins::join_table::JoinTable;

pub struct GeneralJoin<L, R>
    where L: for<'a> DBEntity<'a>,
          R: for<'a> DBEntity<'a> {
    pub left: L,
    pub right: R,
    pub table: JoinTable,
}
