use crate::{
    errors::server_error::{ServerError, ServerResultResponseHtml},
    routes::study::quiz_me::{quiz_me_search_params::QuizMeRouteSearchParams, quiz_page_response::QuizPageResponse},
};
use askama::Template;
use axum::http::StatusCode;

pub async fn quiz_me_route() -> ServerResultResponseHtml {
    let params = QuizMeRouteSearchParams { tag: None };
    let r = QuizPageResponse::from_search_params(params);
    let rendered = r.render().map_err(|e| {
                                  log::error!("Error: {:?}", e);
                                  StatusCode::INTERNAL_SERVER_ERROR
                              })?;
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn quiz_me_route_gets_flashcard() {
        let r = quiz_me_route().await;
        assert!(r.is_ok(), "QuizMe Route throws no error")
        // assert_eq!(result, 4);
    }
}
