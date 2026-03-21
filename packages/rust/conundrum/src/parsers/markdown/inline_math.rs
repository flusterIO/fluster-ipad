use fluster_core_utilities::core_types::component_constants::auto_inserted_component_name::AutoInsertedComponentName;
use winnow::{
    ModalResult, Parser,
    ascii::take_escaped,
    combinator::{delimited, opt},
    token::{one_of, take_till},
};

use crate::{
    lang::runtime::traits::{conundrum_input::ConundrumInput, mdx_component_result::MdxComponentResult},
    parsers::parser_trait::ConundrumParser,
};

#[derive(Debug)]
pub struct InlineMathResult {
    pub body: String,
}

impl MdxComponentResult for InlineMathResult {
    fn to_mdx_component(&self, _: &mut crate::output::parsing_result::mdx_parsing_result::MdxParsingResult) -> String {
        format!("<{}>${}$</{}>",
                AutoInsertedComponentName::AutoInsertedInlineMath,
                self.body,
                AutoInsertedComponentName::AutoInsertedInlineMath,)
    }
}

impl ConundrumParser<InlineMathResult> for InlineMathResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ModalResult<InlineMathResult> {
        let body = delimited("$",
                             // take_escaped returns the raw slice.
                             // If the block is entirely empty (like $$), take_escaped will fail,
                             // so we wrap it in opt().unwrap_or("") if you want to allow empty math blocks.
                             opt(take_escaped(// 1. "Normal" text: Must consume AT LEAST 1 char, and stop at \ or $
                                              take_till(1.., ('\\', '$')),
                                              // 2. Control character: The escape trigger
                                              '\\',
                                              // 3. Escapable: What is legally allowed to follow the \
                                              one_of(['$', '\\']))).map(|s| s.unwrap_or("")),
                             "$").parse_next(input)?;
        Ok(InlineMathResult { body: body.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '$'
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_math_content() {
        let test_content = r#"$e=mc^2$"#;
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res =
            InlineMathResult::parse_input_string(&mut test_data).expect("Parses math block without throwing an error.");
        assert_eq!(res.body, "e=mc^2", "Finds the proper math body when parsing inline math.");
    }
}
