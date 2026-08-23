use fake::Fake;

#[macro_export]
macro_rules! test_crud_functionality {
    ( $entity:ty, $partial:ty, $label:expr ) => {
        use conundrum::ecosystem::db::db_traits::db_entity::DBEntity;
        use conundrum::ecosystem::db::db_traits::into_partial::IntoPartial;
        use std::ops::Index;
        let test_db = $crate::test_utils::get_test_db::get_test_database().await;
        let mut test_data: Vec<$entity> = Vec::new();
        for _ in 0..10 {
            let fake_item: $entity = fake::Faker.fake();
            test_data.push(fake_item);
        }
        <$entity>::save_many(test_data.clone(), &test_db).await.expect(format!("Saves {} values without throwing an error.", $label).as_str());
        let mut saved_items = <$entity>::get_by_predicate(None, None, None, &test_db).await.inspect_err(|e| {
            log::error!("Error: {:#?}", e);
        }).expect(format!("Saves {} values without throwing an error", $label).as_str());
        let mut mutated_items: Vec<$entity> = Vec::new();
        for (i, saved_item) in saved_items.iter().enumerate() {
            let mut new_item: $entity = fake::Faker.fake();
            let saved_item = saved_items.index(i);
            let saved_item_primary_val = saved_item.primary_value();
            new_item.set_primary_value(saved_item_primary_val);
            mutated_items.push(new_item);
        }
        let partials = mutated_items.iter().map(|x| {
            let partial: $partial = x.into_partial();
            partial
        }).collect::<Vec<$partial>>();
        <$entity>::merge_by_primary_key(partials, &std::sync::Arc::clone(&test_db)).await.expect(format!("Failed attempting to upsert {} values", $label).as_str());
        for item in test_data {
            <$entity>::delete_by_primary_key(item.primary_value(), &test_db).await.expect(format!("Deletes {} data without throwing an error", $label).as_str())
        }
        log::info!("Successfully ran crud tests for the {} model.", $label);
    };
}
