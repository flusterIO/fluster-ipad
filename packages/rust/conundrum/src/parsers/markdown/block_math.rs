use serde::Serialize;
use winnow::{
    Parser,
    combinator::{opt, preceded},
    error::ErrMode,
    stream::Stream,
    token::{literal, take_until},
};

use crate::{
    lang::{
        lib::ui::components::markdown::math::props::MathData,
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::ConundrumCompileTarget,
            },
            traits::{
                conundrum_input::{ArcState, ConundrumInput},
                fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult,
                jsx_component_result::JsxComponentResult,
                markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::auto_inserted_component_name::AutoInsertedComponentName,
    parsers::{
        conundrum::logic::string::conundrum_string::ConundrumString,
        markdown::subtypes::inline_id_syntax::inline_id_syntax,
        parser_components::consume_white_space::consume_white_space, parser_trait::ConundrumParser,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct BlockMathResult {
    pub body: ConundrumString,
    pub id: Option<ConundrumString>,
    pub idx: u32,
}

impl PlainTextComponentResult for BlockMathResult {
    fn to_plain_text(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(self.body.0.clone())
    }
}

impl ConundrumComponentResult for BlockMathResult {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.compile_target == ConundrumCompileTarget::PlainText {
            drop(state);
            self.to_plain_text(res)
        } else if state.is_markdown_or_search_or_ai() {
            drop(state);
            self.to_markdown(res)
        } else if state.targets_jsx() {
            drop(state);
            self.to_jsx_component(res)
        } else {
            drop(state);
            self.to_mdx_component(res)
        }
    }
}

impl MarkdownComponentResult for BlockMathResult {
    fn to_markdown(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("$$\n{}\n$$", self.body))
    }
}

impl JsxComponentResult for BlockMathResult {
    fn to_jsx_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        let math_data = MathData { display: true,
                                   idx: Some(self.idx),
                                   id: self.id.as_ref().map(|id| id.0.clone()),
                                   content: self.body.0.clone() };
        Ok(format!("<{} data={{{}}} />",
                   AutoInsertedComponentName::AutoInsertedMathBlock,
                   serde_json::to_string(&math_data).map_err(|e| {
                       println!("Error: {:#?}", e);
                       ErrMode::Backtrack(ConundrumErrorVariant::UserFacingGeneralParserError(
                               ConundrumError::from_msg_and_details("Parser error", "Could not successfully serialize the data provided to a math equation.")
                       ))
                   })?
           ))
    }
}

impl HtmlJsComponentResult for BlockMathResult {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        let math_string = self.body.to_math(false, state.trusted)?;
        Ok(format!("<div className=\"conundrum-math conundrum-math-block\">\n{}\n</div>", math_string))
    }
}

impl MdxComponentResult for BlockMathResult {
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.to_jsx_component(res)
    }
}

impl ConundrumParser<BlockMathResult> for BlockMathResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<BlockMathResult> {
        let start = input.input.checkpoint();
        literal("$$").void().parse_next(input).inspect_err(|_| {
                                                   input.input.reset(&start);
                                               })?;
        let id = opt(preceded(consume_white_space(0..), inline_id_syntax)).parse_next(input)
                                                                          .inspect_err(|_| {
                                                                              input.input.reset(&start);
                                                                          })?;
        let body = take_until(1.., "$$").parse_next(input).inspect_err(|_| {
                                                               input.input.reset(&start);
                                                           })?;
        literal("$$").void().parse_next(input).inspect_err(|_| {
                                                   input.input.reset(&start);
                                               })?;

        // Need to modify state during the parsing phase so that it's available during
        // the rendering phase for the math component indices, otherwise the
        // indices wll only be able to read downwards.
        let mut res = input.state.write_arc();
        let current_last_index = res.eq_count;
        if let Some(_id) = &id {
            res.data.eq_ref_map.insert(_id.clone(), current_last_index);
        }
        res.eq_count += 1;
        Ok(BlockMathResult { body: ConundrumString(body.to_string()),
                             id: id.map(|d| ConundrumString(d.to_string())),
                             idx: current_last_index })
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
    fn parses_block_math_content() {
        let test_content = "$$\n\ne=mc^2\n\n$$";
        let mut test_props = wrap_test_conundrum_content(test_content);
        let res =
            BlockMathResult::parse_input_string(&mut test_props).expect("Parses math block without throwing an error.");
        println!("Res: {:#?}", res);
        assert_eq!(res.body.0, "\n\ne=mc^2\n\n", "Finds the proper math body when parsing inline math.");
    }
}
