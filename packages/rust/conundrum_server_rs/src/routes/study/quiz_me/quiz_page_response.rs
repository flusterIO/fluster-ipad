use conundrum_db::vector::models::academic::question::flashcard::flashcard_model::FlashCardModelInputData;

use crate::routes::study::quiz_me::quiz_me_search_params::QuizMeRouteSearchParams;

#[derive(askama::Template)]
#[template(path = "pages/study/quiz_me_page.html")]
pub struct QuizPageResponse {}

impl QuizPageResponse {
    pub fn from_search_params(params: QuizMeRouteSearchParams) -> Self {
        let test_questions = include_str!("../../../../../conundrum_db/tests/seed_questions.json");
        let parsed: Vec<FlashCardModelInputData<String>> =
            serde_json::from_str(test_questions).expect("Must parse own questions.");
        println!("Parsed: {:#?}", parsed);
        QuizPageResponse {}
    }
}
