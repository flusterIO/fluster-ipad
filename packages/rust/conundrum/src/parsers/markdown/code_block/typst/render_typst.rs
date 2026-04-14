use std::include_str;
use typst_as_lib::TypstEngine;
use winnow::error::ErrMode;

use crate::lang::runtime::state::{
    conundrum_error::ConundrumError,
    conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
};

// static TEMPLATE_FILE: &str = include_str!("./templates/html.typ");

pub fn render_typst(content: &str) -> ConundrumModalResult<String> {
    let engine = TypstEngine::builder().main_file(content).build();

    let doc = engine.compile().output.map_err(|e| {
        eprintln!("Error: {:#?}", e);
        ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Typst compilation error", "Unfortunately we don't have any more information at this time. We're working on integrating with the Typst error handling mechanisms more completely.")))
    })?;

    let html = typst_html::html(&doc).map_err(|e| {
        eprintln!("Error: {:#?}", e);
        ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Typst compilation error", "Unfortunately we don't have any more information at this time. We're working on integrating with the Typst error handling mechanisms more completely.")))
    })?;

    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn renders_typst() {
        let r = render_typst("$ x < y => x gt.eq.not y $").expect("Parses valid typst without throwing an error");
        println!("Response: {}", r);
        // assert_eq!(result, 4);
    }
}


