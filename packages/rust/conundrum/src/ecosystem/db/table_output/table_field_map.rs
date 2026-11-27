use std::collections::HashMap;

use crate::ecosystem::db::table_output::field_table_type::FieldTableType;

/// # TableFieldMap
///
/// A hashmap of the keys in the database model, and the associated field type
/// to be used by the generateTableColumns function on the client side.
pub struct TableFieldMap(HashMap<String, FieldTableType>);
