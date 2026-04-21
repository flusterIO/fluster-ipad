use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            parse_conundrum_string::parse_elements,
            queries::get_title::get_title_group,
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            },
            traits::{
                conundrum_input::{ArcState, ConundrumInput},
                html_js_component_result::HtmlJsComponentResult,
            },
        },
    },
    output::html::{
        glue::gather_glue_assets::{WebGlueAssetData, get_glue_asset_data},
        standalone::standalone_template::StandaloneTemplate,
    },
};
use askama::Template;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::sync::Arc;
use winnow::{Parser, error::ErrMode};

pub enum AnyBlockLevelElement {}

#[derive(Debug)]
pub struct ConundrumDocument(Vec<ParsedElement>);

impl ConundrumDocument {
    pub fn parse_input(input: &mut ConundrumInput) -> ConundrumModalResult<Self> {
        let res = parse_elements.parse_next(input)?;
        Ok(Self(res))
    }

    pub fn elements(&self) -> Vec<ParsedElement> {
        // let y: Vec<ParsedElement> = self.0.iter().map(|n|
        // n.children.0.clone()).flatten().collect(); y
        self.0.clone()
    }

    pub fn compile_multithreaded(&self, state: ArcState) -> ConundrumModalResult<String> {
        let elements = &self.0;
        let res = elements.par_iter()
                          .filter_map(|em| em.to_html_js_component(Arc::clone(&state)).ok())
                          .collect::<Vec<String>>()
                          .join("");
        Ok(res)
    }

    pub fn compile_sync(&self, state: ArcState) -> ConundrumModalResult<String> {
        let mut s = String::from("");
        for em in self.elements() {
            let r = em.to_html_js_component(Arc::clone(&state))?;
            s += r.as_str();
        }
        Ok(s)
    }

    pub fn get_glue(&self, state: ArcState) -> WebGlueAssetData {
        get_glue_asset_data(state)
    }

    /// ### Requirements for a completely standalone html file
    /// - [x] Renders math straight to html and MML.
    /// - [x] Gathers katex css
    /// - [x] Gathers component javascript
    /// - [x] Embeds katex fonts
    /// - [ ] Embeds nerd fonts
    ///
    /// ### If I'm lucky...
    /// - [x] Multi-threaded compilation
    pub fn render_standalone(&self, params: ArcState) -> ConundrumModalResult<String> {
        let glue = self.get_glue(Arc::clone(&params));
        let compiled = self.compile_multithreaded(Arc::clone(&params))?;
        let state = params.read_arc();
        let templ = StandaloneTemplate::new(get_title_group(state.data.content.clone(),
                                                            state.modifiers.clone(),
                                                            state.compile_target.clone()).map(|n| n.title)
                                                                                         .ok(),
                                            compiled,
                                            glue.js,
                                            glue.css,
                                            state.ui_params.clone());
        let rendered_standalone = templ.render().map_err(|_| {

    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })?;
        Ok(rendered_standalone)
    }

    pub fn render_app_embedded(&self, params: ArcState) -> ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}
