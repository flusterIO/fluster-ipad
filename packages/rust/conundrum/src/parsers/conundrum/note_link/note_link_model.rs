use askama::Template;
use serde::Serialize;
use std::sync::Arc;
use typeshare::typeshare;
use winnow::{
    Parser,
    combinator::delimited,
    error::ErrMode,
    token::{literal, take_until},
};

use crate::{
    lang::{
        lib::ui::ui_types::children::Children,
        runtime::{
            parse_conundrum_string::parse_elements,
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::ConundrumCompileTarget,
            },
            traits::{
                conundrum_input::{ArcState, ConundrumInput, get_conundrum_input},
                fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult,
                mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
                state_modifier::ConundrumStateModifier,
            },
        },
    },
    output::{
        general::component_constants::{
            any_component_id::AnyComponentName, auto_inserted_component_name::AutoInsertedComponentName,
            parser_ids::ParserId,
        },
        parsing_result::note_outgoing_link_result::NoteOutgoingLinkResult,
    },
    parsers::{conundrum::note_link::note_link_html_templ::NoteLinkHtmlTemplate, parser_trait::ConundrumParser},
};

#[typeshare]
#[derive(Serialize, Clone, Debug)]
pub struct ParsedOutgoingNoteLink {
    /// The user-defined id of the note which is being linked to.
    pub note_id: String,
    /// The text content of the link. `[The stuff here](note:MyNote)`
    pub content: Children,
    /// The full content of the input string that represents this outgoing note
    /// link.
    pub full_match: String,
}

impl ConundrumStateModifier<ParsedOutgoingNoteLink> for ParsedOutgoingNoteLink {
    fn set_state(res: ArcState, _data: Option<ParsedOutgoingNoteLink>) {
        let data = _data.unwrap();
        let mut state = res.write_arc();
        if !state.data.contains_outgoing_link(&data.note_id) {
            state.data.outgoing_links.push(NoteOutgoingLinkResult { link_to_note_id: data.note_id.clone() });
        }
    }
}

impl PlainTextComponentResult for ParsedOutgoingNoteLink {
    fn to_plain_text(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(self.full_match.clone())
    }
}

impl HtmlJsComponentResult for ParsedOutgoingNoteLink {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let tmpl = NoteLinkHtmlTemplate { children: self.content.render(res)?,
                                          note_id: self.note_id.clone() };
        tmpl.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl ConundrumComponentResult for ParsedOutgoingNoteLink {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.compile_target == ConundrumCompileTarget::PlainText {
            drop(state);
            self.to_plain_text(res)
        } else {
            drop(state);
            self.to_mdx_component(res)
        }
    }
}

impl MdxComponentResult for ParsedOutgoingNoteLink {
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.data.ignore_all_parsers {
            return Ok(self.full_match.clone());
        }

        if state.data
                .front_matter
                .as_ref()
                .is_some_and(|fm| fm.ignored_parsers.iter().any(|x| x == &ParserId::NoteLink.to_string()))
        {
            return Ok(self.full_match.clone());
        }

        Ok(format!("<NoteLink id=\"{}\">{}</NoteLink>", self.note_id, self.content.render(Arc::clone(&res))?))
    }
}

impl ConundrumParser<ParsedOutgoingNoteLink> for ParsedOutgoingNoteLink {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<ParsedOutgoingNoteLink> {
        let ((display_text, note_id), full_match) = (
            delimited('[', take_until(1.., ']'), ']'),
            delimited(literal("(note:"), take_until(1.., ")"), ')')
        )
        .with_taken()
        .parse_next(input)?;

        let mut state = input.state.write_arc();
        state.data.append_embeddable_component(&AnyComponentName::AutoInserted(AutoInsertedComponentName::NoteLink));
        drop(state);
        let mut nested_state = ConundrumInput { input: display_text,
                                                state: Arc::clone(&input.state) };
        let res = parse_elements(&mut nested_state)?;

        let l = ParsedOutgoingNoteLink { note_id: note_id.to_string(),
                                         content: Children(res),
                                         full_match: full_match.to_string() };
        ParsedOutgoingNoteLink::set_state(Arc::clone(&input.state), Some(l.clone()));
        Ok(l)
    }

    fn matches_first_char(char: char) -> bool {
        char == '['
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parsed_link() {
        let test_content = "[This is my link](note:myNote)";
        let mut test_data = wrap_test_conundrum_content(test_content);

        let res =
            ParsedOutgoingNoteLink::parse_input_string(&mut test_data).expect("Parses outgoing link successfully.");

        assert!(test_data.input.is_empty(), "NoteLink returns the proper left over text.");

        assert!(res.content.render(Arc::clone(&test_data.state)).expect("renders children without throwing an error.")
                == "This is my link",
                "NoteLink finds the proper content.");
        assert!(res.note_id == "myNote", "NoteLink finds the proper note id.");
    }
}
