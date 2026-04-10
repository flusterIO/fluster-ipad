use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use typeshare::typeshare;
use winnow::{
    Parser,
    ascii::{alpha1, line_ending},
    combinator::{alt, eof},
    stream::Stream,
    token::literal,
};

use crate::{
    embedded::{
        embedded_component_docs::EmbeddedComponentDocs,
        embedded_in_content_docs::EmbeddedInContentDocs,
        in_content_documentation_id::{InContentDocumentationFormat, InContentDocumentationId},
    },
    lang::{
        lib::ui::ui_types::children::Children,
        runtime::{
            parse_conundrum_string::parse_elements,
            state::{
                conundrum_error_variant::ConundrumModalResult,
                parse_state::{ConundrumModifier, ParseState},
            },
            traits::{
                conundrum_input::{ConundrumInput, get_conundrum_input},
                fluster_component_result::ConundrumComponentResult,
                mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        component_name_id_map::COMPONENT_NAME_ID_MAP, component_names::EmbeddableComponentName,
        documentation_component_name::DocumentationComponentName,
    },
    parsers::parser_trait::ConundrumParser,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct ParsedInspectionRequest {
    /// An EmbeddableComponentName or InContentDocumentationId
    pub keyword: String,
    pub level: u8, // 1 for '?', 2 for '??'
    pub full_match: String,
}

impl PlainTextComponentResult for ParsedInspectionRequest {
    fn to_plain_text(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}

impl ConundrumComponentResult for ParsedInspectionRequest {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MdxComponentResult for ParsedInspectionRequest {
    fn to_mdx_component(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
        if let Some(depth) = match self.level {
            1 => Some(InContentDocumentationFormat::Short),
            2 => Some(InContentDocumentationFormat::Full),
            _ => None,
        } {
            if let Some(doc_id) = InContentDocumentationId::iter().find(|x| x.to_string() == self.keyword) {
                let body_as_string = EmbeddedInContentDocs::get_incontent_docs_by_id(&doc_id, &depth);
                // let mut new_input = get_conundrum_input(body_as_string.as_str(),
                // res.modifiers.clone()); let c = parse_elements(&mut
                // new_input)?; let rendered_body = Children(c).render(res)?;
                return Ok(format!("\n<{} inContentId=\"{}\" format=\"{}\">\n{}\n</{}>\n",
                                  DocumentationComponentName::InContentDocumentationContainer,
                                  doc_id,
                                  depth,
                                  body_as_string,
                                  DocumentationComponentName::InContentDocumentationContainer,));
            } else if let Some(comp_name) = EmbeddableComponentName::iter().find(|x| x.to_string() == self.keyword)
                                                                           .map(|component_name| {
                                                                               COMPONENT_NAME_ID_MAP
                        .get(&component_name)
                        .unwrap_or_else(|| panic!("All components must have documentation: {}", &component_name))
                                                                           })
            {
                let body_as_string = EmbeddedComponentDocs::get_incontent_docs_by_id(comp_name, &depth);
                // let mut new_input = get_conundrum_input(body_as_string.as_str(),
                // res.modifiers.clone()); let c = parse_elements(&mut
                // new_input)?; let rendered_body = children(c).render(res)?;
                return Ok(format!("\n<{} componentName=\"{}\" format=\"{}\">\n{}\n</{}>\n",
                                  DocumentationComponentName::InContentDocumentationContainer,
                                  comp_name,
                                  depth,
                                  body_as_string,
                                  DocumentationComponentName::InContentDocumentationContainer,));
            }
        }
        Ok(self.full_match.clone())
    }
}

impl ConundrumParser<ParsedInspectionRequest> for ParsedInspectionRequest {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<ParsedInspectionRequest> {
        let start = input.input.checkpoint();
        let ((keyword, marks), full_match) =
            (|input: &mut ConundrumInput<'a>| {
                let (keyword, marks, _) = (alpha1.verify(|s: &str| {
                                                     for name in EmbeddableComponentName::iter() {
                                                         if name.to_string() == s {
                                                             return true;
                                                         }
                                                     }
                                                     for id in InContentDocumentationId::iter() {
                                                         if id.to_string() == s {
                                                             return true;
                                                         }
                                                     }
                                                     false
                                                 }),
                                           alt((literal("??"), literal("?"))),
                                           alt((line_ending, eof)))
                                                                   .parse_next(input)
                                                                   .inspect_err(|_| {
                                                                       input.input.reset(&start);
                                                                   })?;
                let _ = alt((line_ending, literal(""))).parse_next(input).inspect_err(|_| {
                                                                              input.input.reset(&start);
                                                                          })?;
                Ok((keyword, marks))
            }).with_taken()
              .parse_next(input)?;

        let level = if marks == "??" {
            2
        } else {
            1
        };
        let mut state = input.state.borrow_mut();
        state.data.ignore_all_parsers = true;
        Ok(ParsedInspectionRequest { keyword: keyword.to_string(),
                                     level,
                                     full_match: full_match.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '`'
    }
}
