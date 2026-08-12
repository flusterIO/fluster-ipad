#[macro_export]
macro_rules! topic_join {
    ( $id_type:ty, $struct_name:ident, $id_key:ident ) => {
        use std::sync::Arc;

        use conundrum::ecosystem::db::traits::db_entity::DBSchema;

        use $crate::vector::{database::db_traits::db_field::DatabaseField};

        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
        pub struct $struct_name {
            pub topic_value: String,
            pub $id_key: $id_type,
        }

        impl<'a> DBSchema<'a> for $struct_name {
            fn arrow_fields(
                )
                -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
            {
                Ok(vec![Arc::new(String::field_definition("topic_value", false)),
                        Arc::new(<$id_type>::field_definition(stringify!($id_key), false))])
            }
        }
    };
}
