use conundrum::ecosystem::db::tables::DatabaseTable;
use std::{collections::HashMap, hash::Hash};

#[allow(non_camel_case_types)]
#[derive(Clone, strum_macros::Display, PartialEq, Eq)]
pub enum JoinTable {
    #[strum(to_string = "qa_pair_tag")]
    QAPair_Tag,
}

impl Hash for JoinTable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl JoinTable {
    pub fn to_join_map() -> HashMap<JoinTable, (DatabaseTable, DatabaseTable)> {
        // PERFORMANCE: Move this to phf.
        let mut map: HashMap<JoinTable, (DatabaseTable, DatabaseTable)> = HashMap::new();
        map.insert(JoinTable::QAPair_Tag, (DatabaseTable::QAPair, DatabaseTable::Tag));
        map
    }
}
