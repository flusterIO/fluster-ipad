use fake::Fake;

#[macro_export]
macro_rules! test_crud_functionality {
    ( $entity:ty, $label:expr ) => {
        {
        use conundrum::ecosystem::db::db_traits::db_entity::DBEntity;
        let test_db = $crate::test_utils::get_test_db::get_test_database().await;
        let mut test_data: Vec<$entity> = Vec::new();
        for _ in 0..10 {
            let fake_item: $entity = fake::Faker.fake();
            test_data.push(fake_item);
        }
        <$entity>::save_many(test_data.clone(), &test_db).await.expect(format!("Saves {} values without throwing an error.", $label).as_str());
        for item in test_data {
            <$entity>::delete_by_primary_key(item.primary_value(), &test_db).await.expect(format!("Deletes {} data without throwing an error", $label).as_str())
        }
        }
    };
}
