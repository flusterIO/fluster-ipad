pub mod errors;
pub mod routes;

use axum::{
    Router,
    routing::{get, post},
};
use conundrum::ecosystem::environment_variables::cdrm_env_variable::{CdrmEnvVariable, DEFAULT_CDRM_SERVER_PORT};

use crate::routes::{
    route_enum::RouteEnum,
    study::{
        quiz_me::quiz_me_route::quiz_me_route, random_question::get_random_question,
        save_flashcard::save_flashcard_route,
    },
};

async fn handler() -> &'static str {
    "Hello, Axum!"
}

#[tokio::main]
pub async fn main() {
    let app = Router::new().route(RouteEnum::Math_TexToSvg.to_string().as_str(), get(handler))
                           .route(RouteEnum::Study_RandomQuestion.to_string().as_str(), get(get_random_question))
                           .route(RouteEnum::Study_QuizMe.to_string().as_str(), get(quiz_me_route))
                           .route(RouteEnum::Study_SaveFlashcard.to_string().as_str(), post(save_flashcard_route));
    let port = match CdrmEnvVariable::ServerPort.read() {
        Ok(c) => {
            let n: Result<u32, _> = c.parse();
            n.unwrap_or(*DEFAULT_CDRM_SERVER_PORT)
        }
        Err(err) => {
            println!("Error: {}", err);
            *DEFAULT_CDRM_SERVER_PORT
        }
    };
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
