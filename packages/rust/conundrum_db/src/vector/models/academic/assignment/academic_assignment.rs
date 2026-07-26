use std::sync::Arc;

use arrow_array::{RecordBatch, StringArray};

use crate::vector::{
    database::db_types::db_entity::DBEntity,
    models::{
        academic::assignment::milestone::Milestone,
        date_time::{date_time::DateTime, due_date::DueDate},
        primitives::db_id::DatabaseId,
        taggables::taggables::Taggables,
    },
};

use conundrum::ecosystem::{
    db::traits::database_field_representable::DatabaseFieldRepresentable, error_handling::db_error::DatabaseError,
};

pub struct Assignment {
    pub id: DatabaseId,
    pub label: String,
    pub desc: Option<String>,
    pub taggables: Taggables,
    pub milestones: Vec<Milestone>,
    pub due_at: Option<DueDate>,
    pub ctime: DateTime,
    pub utime: DateTime,
}

// impl DBEntity for Assignment {
//     fn primary_key_value(&self) -> DatabaseId {
//         self.id
//     }

//     fn to_arrow_schema() -> Arc<Schema> {
//         Arc::new(Schema::new(vec![Field::new("id", DataType::Utf8, false),
//                                   Field::new("label", DataType::Utf8, false),
//                                   Field::new("desc", DataType::Utf8, true),
//                                   Field::new("ctime", DataType::Date64,
// false),                                   Field::new("utime",
// DataType::Date64, false),]))     }

//     fn to_record_batch(items: Vec<Self>) ->
// DatabaseResult<arrow_array::RecordBatch> {         let mut id = Vec::new();
//         let mut label = Vec::new();
//         let mut desc = Vec::new();
//         let mut ctime = Vec::new();
//         let mut utime = Vec::new();
//         for item in items {
//             id.push(item.id.to_db_representation());
//             label.push(item.label);
//             desc.push(item.desc);
//             ctime.push(item.ctime.to_db_representation());
//             utime.push(item.utime.to_db_representation());
//         }

//         RecordBatch::try_new(Self::to_arrow_schema(),
//                              vec![Arc::new(StringArray::from(id)),
//                                   Arc::new(StringArray::from(label)),
//                                   Arc::new(StringArray::from(desc)),
//                                   Arc::new(StringArray::from(ctime)),
//
// Arc::new(StringArray::from(utime)),]).map_err(|e| {
// DatabaseError::FailToSerialize(e.to_string())
// })     }
// }
