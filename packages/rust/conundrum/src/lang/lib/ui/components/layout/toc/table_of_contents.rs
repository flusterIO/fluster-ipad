use askama::Template;
use serde::Serialize;
use winnow::error::ErrMode;

use crate::{
    lang::{
        lib::ui::{
            components::{
                component_trait::ConundrumComponent, layout::toc::table_of_contents_html_templ::TocHtmlTemplate,
            },
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            },
            traits::{
                conundrum_input::ArcState, fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult, jsx_component_result::JsxComponentResult,
                markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::{
        general::component_constants::{any_component_id::AnyComponentName, component_names::EmbeddableComponentName},
        html::dom::dom_id::DOMId,
    },
    parsers::conundrum::logic::bool::boolean::ConundrumBoolean,
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct TableOfContents {
    pub expanded: Option<ConundrumBoolean>,
    pub id: DOMId,
}

// impl TableOfContents {
//     pub fn get_toc_with_tab_depth() -> Vec<MarkdownHeadingStringifiedResult>
// }

impl MarkdownComponentResult for TableOfContents {
    fn to_markdown(&self,
                   res: ArcState)
                   -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        let mut s = String::from("");
        let mut last_depth = 0;
        let mut last_tab_depth = 0;
        let state = res.read_arc();
        for item in &state.data.toc {
            if item.depth > last_depth {
                last_tab_depth += 1;
            } else if item.depth < last_depth {
                last_tab_depth -= 1;
            }
            last_depth = item.depth;
            let mut tab_string = String::from("");
            for _ in 0..last_tab_depth {
                tab_string += "    ";
            }
            s += &format!("{} {}", tab_string, item.content);
        }
        Ok(s)
    }
}

impl HtmlJsComponentResult for TableOfContents {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        let toc = state.data.toc.clone();
        let templ = TocHtmlTemplate { toc,
                                      expanded: self.expanded.map(|n| n.0).unwrap_or_else(|| false),
                                      id: self.id.clone() };
        templ.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl PlainTextComponentResult for TableOfContents {
    fn to_plain_text(&self,
                     _: ArcState)
                     -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}

impl JsxComponentResult for TableOfContents {
    fn to_jsx_component(&self,
                        res: ArcState)
                        -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        let state = res.read_arc();
        let json_string = serde_json::to_string(&state.data.toc).map_err(|e| {
            eprintln!("Error: {:#?}", e);
            ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("JSON error", "Conundrum could not parse the table of contents to json. This is almost surely a bug on my end, and I'm working on it.")))
        })?;
        Ok(format!("<{} toc={{{}}} {}/>", EmbeddableComponentName::Toc, json_string, match self.expanded {
            Some(x) => {
                if x.0 {
                    String::from("expanded")
                } else {
                    String::from("")
                }
            }
            None => {
                String::from("")
            }
        }))
    }
}

impl ConundrumComponentResult for TableOfContents {
    fn to_conundrum_component(&self,
                              res: ArcState)
                              -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String>
    {
        let state = res.read_arc();
        if state.is_markdown_or_search_or_ai() {
            drop(state);
            self.to_markdown(res)
        } else if state.is_plain_text() {
            drop(state);
            self.to_plain_text(res)
        } else {
            drop(state);
            self.to_jsx_component(res)
        }
    }
}

impl ConundrumComponent for TableOfContents {
    fn get_component_id() -> crate::output::general::component_constants::any_component_id::AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Toc)
    }

    fn from_props(props: crate::parsers::conundrum::logic::object::object::ConundrumObject,
                  _: Option<Vec<crate::lang::elements::parsed_elements::ParsedElement>>,
                  state: ArcState)
                  -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<Self>
        where Self: Sized {
        let expanded = ConundrumBoolean::from_jsx_props(&props, "expanded").ok();
        let mut borrowed_state = state.write_arc();
        let id = borrowed_state.dom.new_id();
        drop(borrowed_state);
        Ok(TableOfContents { expanded,
                             id })
    }
}
