use axum::response::Response;

pub struct GetRandomQuestionRequestData {
    pub tags: Option<Vec<String>>,
}

pub async fn get_random_question() -> Response<String> {
    Response::new(String::from("Here"))
}

pub struct GetRandomQuestion {}
