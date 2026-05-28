use askama::Template;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use winnow::error::ErrMode;
use winnow::{Parser, stream::Stream};

use crate::lang::runtime::state::conundrum_error::ConundrumError;
use crate::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;
use crate::lang::runtime::traits::as_template::AsTemplate;
use crate::lang::runtime::traits::conundrum_input::ArcState;
use crate::lang::runtime::traits::html_js_component_result::HtmlJsComponentResult;
use crate::parsers::markdown::lists::task_list::task_list_item::checkbox::checkbox;
use crate::parsers::markdown::lists::task_list::task_list_item::task_list_completion_indicator::TaskListCompletionToken;
use crate::parsers::markdown::lists::task_list::task_list_item::task_list_item_html_templ::UnorderedTaskListItemHtmlTemplate;
use crate::parsers::markdown::lists::unordered::unordered_line_item::unordered_line_item_model::parse_list_item_children;
use crate::{
    lang::{
        lib::ui::ui_types::children::Children,
        runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    },
    parsers::{
        markdown::lists::list_shared::line_item_marker::line_item_marker, parser_trait::ConundrumParser,
        parsers_shared::space_or_tab::spaces_only,
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnorderedTaskListItem {
    pub heading: Children,
    pub body: Option<Children>,
    pub status: TaskListCompletionToken,
}

impl AsTemplate<UnorderedTaskListItemHtmlTemplate> for UnorderedTaskListItem {
    fn as_template(&self, state: ArcState) -> ConundrumModalResult<UnorderedTaskListItemHtmlTemplate> {
        let r = UnorderedTaskListItemHtmlTemplate { body: match &self.body {
                                                        Some(b) => Some(b.render(Arc::clone(&state))?),
                                                        None => None,
                                                    },
                                                    status: self.status.clone(),
                                                    heading: self.heading.render(Arc::clone(&state))? };
        Ok(r)
    }
}

impl HtmlJsComponentResult for UnorderedTaskListItem {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let templ = self.as_template(res)?;
        templ.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl ConundrumParser<UnorderedTaskListItem> for UnorderedTaskListItem {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<UnorderedTaskListItem> {
        let start = input.input.checkpoint();

        spaces_only(0..=3).parse_next(input).inspect_err(|_| {
                                                 input.input.reset(&start);
                                             })?;

        line_item_marker.parse_next(input).inspect_err(|_| {
                                               input.input.reset(&start);
                                           })?;

        spaces_only(1..=3).parse_next(input).inspect_err(|_| {
                                                 input.input.reset(&start);
                                             })?;

        let status = checkbox.parse_next(input).inspect_err(|_| {
                                                    input.input.reset(&start);
                                                })?;

        let (heading, body) = parse_list_item_children.parse_next(input).inspect_err(|_| {
                                                                             input.input.reset(&start);
                                                                         })?;

        Ok(UnorderedTaskListItem { heading,
                                   body,
                                   status })
    }

    fn matches_first_char(char: char) -> bool {
        char == '-' || char == '*' || char == '+'
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::{wrap_test_content::wrap_test_conundrum_content, write_test_ast::write_test_ast};

    use super::*;

    #[test]
    fn matches_plain_list_item() {
        let test_content = "- My test item here";
        let mut input = wrap_test_conundrum_content(test_content);
        let res = UnorderedTaskListItem::parse_input_string(&mut input).expect("Parses plain list item without throwing an error.");
        let templ = res.as_template(Arc::clone(&input.state)).expect("Outputs list item template as expected");
        assert!(templ.heading == "My test item here", "Finds the proper heading");
        assert!(templ.body.is_none(), "Finds no body when no body is present");
    }

    #[test]
    fn matches_plain_list_item_with_body() {
        let test_content = r#"- My test item here
  My body goes here!"#;
        let mut input = wrap_test_conundrum_content(test_content);
        let res = UnorderedTaskListItem::parse_input_string(&mut input).expect("Parses plain list item without throwing an error.");
        let templ = res.as_template(Arc::clone(&input.state)).expect("Outputs list item template as expected");
        assert!(templ.heading == "My test item here", "Finds the proper heading");
        assert!(templ.body.is_some_and(|b| b == "My body goes here!"), "Finds the proper body");
    }

    #[test]
    fn matches_plain_list_item_with_body_and_empty_lines() {
        let test_content = r#"- My test item here
  My body goes here!

  My body continues here!"#;
        let mut input = wrap_test_conundrum_content(test_content);
        let res = UnorderedTaskListItem::parse_input_string(&mut input).expect("Parses plain list item without throwing an error.");
        let templ = res.as_template(Arc::clone(&input.state)).expect("Outputs list item template as expected");
        println!("Template: {:#?}", templ);
        assert!(templ.heading == "My test item here", "Finds the proper heading");
        assert!(templ.body.is_some_and(|b| b == "<p>My body goes here!</p><p>My body continues here!</p>"),
                "Finds the proper list item body");
        write_test_ast(test_content, "simple-list-item").expect("Writes ast successfully.");
    }
}
