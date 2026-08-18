#[macro_export]
macro_rules! renamed_enum {
    ( $item_type:ty ) => {
        use conundrum::ecosystem::error_handling::db_error::DatabaseError;
        use strum::IntoEnumIterator;
        impl TryFrom<String> for $item_type {
            type Error = conundrum::ecosystem::error_handling::db_error::DatabaseError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                for k in <$item_type>::iter() {
                    if k.to_string() == value {
                        return Ok(k);
                    }
                }
                return Err(DatabaseError::SerializationError);
            }
        }

        impl Into<String> for $item_type {
            fn into(self) -> String {
                self.to_string()
            }
        }
    };
}
