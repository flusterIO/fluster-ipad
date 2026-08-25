use crate::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};

pub trait QuotedString {
    fn to_quoted_string(&self) -> DatabaseResult<String>;
}

impl QuotedString for String {
    fn to_quoted_string(&self) -> DatabaseResult<String> {
        let s = serde_json::to_string(&self).map_err(|e| {
                                                log::error!("Serialization Error: {:#?}", e);
                                                DatabaseError::SerializationError
                                            })?;
        Ok(s)
    }
}
