use crate::lang::runtime::{
    state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput,
};
use crate::parsers::parser_components::consume_white_space::consume_white_space;
use winnow::Parser;
use winnow::combinator::delimited;
use winnow::token::literal;

pub fn react_closing_tag_parser_by_name(component_name: &str)
                                        -> impl Fn(&mut ConundrumInput) -> ConundrumModalResult<()> {
    move |input| {
        let _ = delimited(literal("</"),
                          delimited(consume_white_space(0..), literal(component_name), consume_white_space(0..)),
                          '>').parse_next(input);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use parking_lot::RwLock;

    use crate::lang::runtime::state::parse_state::ParseState;

    use super::*;

    #[test]
    fn parses_react_closing_tag() {
        let test_input = r#"</ Tags
            >"#;
        let mut input = ConundrumInput { input: test_input,
                                         state: Arc::new(RwLock::new(ParseState::default())) };
        react_closing_tag_parser_by_name("Tags").parse_next(&mut input).expect("Parses a closing tag that is valid");
        // assert_eq!(result, 4);
    }
}
