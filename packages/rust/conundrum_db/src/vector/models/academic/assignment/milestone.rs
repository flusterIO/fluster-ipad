use std::sync::Arc;

use arrow_array::{RecordBatch, StringArray};
use conundrum::ecosystem::{
    db::traits::database_field_representable::DatabaseFieldRepresentable,
    error_handling::db_error::{DatabaseError, DatabaseResult},
};

use crate::vector::{
    database::db_types::db_entity::DBEntity,
    models::{
        academic::assignment::assignment_status::AssignmentStatus, date_time::due_date::DueDate,
        primitives::db_id::DatabaseId,
    },
};

pub struct Milestone {
    pub id: DatabaseId,
    pub label: String,
    pub desc: Option<String>,
    pub status: AssignmentStatus,
    pub due_at: Option<DueDate>,
}

// impl DBEntity for Milestone {
//     fn primary_key_value(&self) ->
// crate::vector::models::primitives::db_id::DatabaseId {         self.id
//     }

//     fn to_arrow_schema() ->
// std::sync::Arc<lancedb::arrow::arrow_schema::Schema> {
//         Arc::new(Schema::new(vec![Field::new("id", DataType::Utf8, false),
//                                   Field::new("label", DataType::Utf8, false),
//                                   Field::new("desc", DataType::LargeUtf8,
// true),                                   Field::new("status",
// DataType::LargeUtf8, true),]))     }

//     fn to_record_batch(items: Vec<Self>) ->
// DatabaseResult<arrow_array::RecordBatch> {         let mut id = Vec::new();
//         let mut label = Vec::new();
//         let mut desc = Vec::new();
//         let mut status = Vec::new();
//         for item in items {
//             id.push(item.id.to_db_representation());
//             label.push(item.label);
//             desc.push(item.desc);
//             status.push(item.status.to_string());
//         }

//         RecordBatch::try_new(Self::to_arrow_schema(),
//                              vec![Arc::new(StringArray::from(id)),
//                                   Arc::new(StringArray::from(label)),
//                                   Arc::new(StringArray::from(desc)),
//                                   Arc::new(StringArray::from(status)),
//                              ]).map_err(|e| {
//
// DatabaseError::FailToSerialize(e.to_string())
// })     }
// }
