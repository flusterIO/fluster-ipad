#![recursion_limit = "1024"]

use conundrum::ecosystem::db::db_traits::db_entity::DBSchema;
use conundrum::ecosystem::db::db_traits::entity_crud::EntityCRUD;
use conundrum::ecosystem::db::tables::DatabaseTable;
use conundrum_db::test_utils::get_test_db::get_test_database;
use conundrum_db::vector::models::academic::question::flashcard::flashcard_entity::FlashCardEntity;
use conundrum_db::vector::models::academic::question::flashcard::flashcard_model::FlashCardModelStringAnswerInputData;

#[tokio::test]
pub async fn test_seed_cdrm_database() {
    let test_data = include_str!("./seed_questions.json");
    let seed_questions: Vec<FlashCardModelStringAnswerInputData> =
        serde_json::from_str(test_data).expect("Fails to deserialize study database");
    let db = get_test_database().await;
    let tbl = db.get_table(&DatabaseTable::QAPair).await.expect("Failed getting table.");
    let questions = seed_questions.iter()
                                  .map(|x| {
                                      let q: FlashCardEntity = x.clone().into();
                                      q
                                  })
                                  .collect::<Vec<FlashCardEntity>>();

    <FlashCardEntity as EntityCRUD<<FlashCardEntity as DBSchema>::PartialUpdateType>>::save_many(questions, db.clone()).await
                                                                         .expect("Failed saving flashcard entities");
    println!("Seeded database flashcard questions.");
}
