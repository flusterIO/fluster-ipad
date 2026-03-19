use fluster_core_utilities::core_types::component_constants::auto_inserted_component_name::AutoInsertedComponentName;
use winnow::{
    ModalResult, Parser,
    combinator::delimited,
    error::StrContext,
    token::{literal, take_until},
};

use crate::{lang::runtime::traits::mdx_component_result::MdxComponentResult, parsers::parser_trait::ConundrumParser};

pub struct InlineMathResult {
    pub body: String,
}

impl MdxComponentResult for InlineMathResult {
    fn to_mdx_component(&self,
                        res: &mut crate::output::parsing_result::mdx_parsing_result::MdxParsingResult)
                        -> String {
        format!("\n<{}>\n{}\n</{}>\n",
                AutoInsertedComponentName::AutoInsertedBlockMath,
                self.body,
                AutoInsertedComponentName::AutoInsertedBlockMath,)
    }
}

impl ConundrumParser<InlineMathResult> for InlineMathResult {
    fn parse_input_string(input: &mut &str) -> ModalResult<InlineMathResult> {
        let body =
            delimited('$', take_until(0.., '$').context(StrContext::Label("Inline Math")), '$').parse_next(input)?;
        Ok(InlineMathResult { body: body.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '$'
    }
}
