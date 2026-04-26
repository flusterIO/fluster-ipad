use askama::Template;
use serde::Serialize;
use std::sync::Arc;
use winnow::combinator::opt;
use winnow::error::ErrMode;
use winnow::token::take;
use winnow::{Parser, stream::Stream};

use crate::lang::runtime::parse_conundrum_string::parse_elements;
use crate::lang::runtime::state::conundrum_error::ConundrumError;
use crate::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;
use crate::lang::runtime::traits::as_template::AsTemplate;
use crate::lang::runtime::traits::conundrum_input::ArcState;
use crate::lang::runtime::traits::html_js_component_result::HtmlJsComponentResult;
use crate::parsers::markdown::lists::unordered::unordered_line_item::unorder_list_html_templ::UnorderedListItemHtmlTemplate;
use crate::parsers::parsers_shared::indentation_handling::repeated_indented_lines::{
    join_indentend_line_types, repeated_indented_lines,
};
use crate::parsers::parsers_shared::shared_enums::line_terminator::LineTerminator;
use crate::{
    lang::{
        lib::ui::ui_types::children::Children,
        runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    },
    parsers::{
        markdown::lists::list_shared::line_item_marker::line_item_marker,
        parser_trait::ConundrumParser,
        parsers_shared::{shared_enums::line_terminator::until_line_terminator, space_or_tab::spaces_only},
    },
};

/// ## Example Usage
///
/// ```mdx
/// - My normal list item
/// - My cool item's heading
///   My cool list item's body, indented either 2 spaces or 1 tab.
///   Unlike other markdown based parsers, Conundrum seperates the list
///   item's body from it's content in the AST, letting developers treat
///   each piece of data accordingly.
///    - This will be a nested item, indented 4 spaces.
/// ```
#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct UnorderedListItem {
    pub heading: Children,
    pub body: Option<Children>,
}

impl AsTemplate<UnorderedListItemHtmlTemplate> for UnorderedListItem {
    fn as_template(&self, state: ArcState) -> ConundrumModalResult<UnorderedListItemHtmlTemplate> {
        let r = UnorderedListItemHtmlTemplate { body: match &self.body {
                                                    Some(b) => Some(b.render(Arc::clone(&state))?),
                                                    None => None,
                                                },
                                                heading: self.heading.render(Arc::clone(&state))? };
        Ok(r)
    }
}

impl HtmlJsComponentResult for UnorderedListItem {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let templ = self.as_template(res)?;
        templ.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl ConundrumParser<UnorderedListItem> for UnorderedListItem {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<UnorderedListItem> {
        let start = input.input.checkpoint();

        spaces_only(0..=3).parse_next(input).inspect_err(|_| {
                                                 input.input.reset(&start);
                                             })?;

        line_item_marker.parse_next(input).inspect_err(|_| {
                                               input.input.reset(&start);
                                           })?;

        spaces_only(1).parse_next(input).inspect_err(|_| {
                                             input.input.reset(&start);
                                         })?;

        let (title_line_chars, _): (Vec<String>, LineTerminator) =
            until_line_terminator::<'_, String>(0.., |nested_input| {
                take(1usize).map(|_| String::from("")).parse_next(nested_input)
            }).parse_next(input)
              .inspect_err(|_| {
                  input.input.reset(&start);
              })?;

        let body = opt(repeated_indented_lines(1..)).parse_next(input)
                                                    .inspect_err(|_| {
                                                        input.input.reset(&start);
                                                    })?
                                                    .map(join_indentend_line_types);
        let body_children = match body {
            Some(b) => {
                let mut new_input = ConundrumInput { input: &b,
                                                     state: Arc::clone(&input.state) };
                let c = parse_elements(&mut new_input)?;
                Some(Children(c))
            }
            None => None,
        };

        let title_string = title_line_chars.join("");
        let mut title_input = ConundrumInput { input: &title_string,
                                               state: Arc::clone(&input.state) };

        let title_children = parse_elements(&mut title_input)?;

        Ok(UnorderedListItem { heading: Children(title_children),
                               body: body_children })
    }

    fn matches_first_char(char: char) -> bool {
        char == '-' || char == '*' || char == '+'
    }
}
