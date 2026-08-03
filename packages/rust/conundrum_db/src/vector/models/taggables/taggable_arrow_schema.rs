#[macro_export]
macro_rules! taggable_arrow_schema {
    () => {{
        let val = CaseInsensitiveString::field_definition("value", false);
        Arc::new(lancedb::arrow::arrow_schema::Schema::new(vec![val,
                                                                TagLocation::field_definition("location", false),
                                                                DateTime::field_definition("ctime", false),
                                                                DateTime::field_definition("last_access", false),]))
    }};
}
