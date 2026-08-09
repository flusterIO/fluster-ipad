#[macro_export]
macro_rules! get_taggable_recordbatch {
    ($items:expr) => {{
        let mut values: Vec<String> = Vec::new();
        let mut values_lc: Vec<String> = Vec::new();
        let mut locations: Vec<String> = Vec::new();
        let mut ctimes: Vec<i64> = Vec::new();
        let mut last_accesss: Vec<i64> = Vec::new();

        for item in $items {
            let (val, val_lc) = item.value.to_db_representation();
            values.push(val);
            values_lc.push(val_lc);
            locations.push(item.location.to_string());
            ctimes.push(item.ctime.to_db_representation());
            last_accesss.push(item.last_access.to_db_representation());
        }

        let values_array = arrow_array::StringArray::from(values);
        let values_lc_array = arrow_array::StringArray::from(values_lc);
        let locations_array = arrow_array::StringArray::from(locations);
        let ctimes_array = arrow_array::TimestampMillisecondArray::from(ctimes).with_timezone("UTC");
        let last_access_array = arrow_array::TimestampMillisecondArray::from(last_accesss).with_timezone("UTC");

        let schema = Self::schema()?;

        let arc_schema = std::sync::Arc::new(schema);

        arrow_array::RecordBatch::try_new(arc_schema,
                                          vec![Arc::new(values_array),
                                               Arc::new(values_lc_array),
                                               Arc::new(locations_array),
                                               Arc::new(ctimes_array),
                                               Arc::new(last_access_array)]).map_err(|e| {
            println!("Error: {:?}", e);
            log::error!("Error: {:?}", e);
            conundrum::ecosystem::error_handling::db_error::DatabaseError::SerializationError
        })
    }};
}
