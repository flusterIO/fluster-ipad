use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            parse_conundrum_string::parse_elements,
            queries::get_title::get_title_group,
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::{ConundrumModifier, ParseState},
            },
            traits::{conundrum_input::ConundrumInput, html_js_component_result::HtmlJsComponentResult},
        },
    },
    output::html::{
        glue::gather_glue_assets::{WebGlueAssetData, get_glue_asset_data},
        standalone::standalone_template::StandaloneTemplate,
    },
};
use askama::Template;
use winnow::{Parser, error::ErrMode};

pub enum AnyBlockLevelElement {}

pub struct ConundrumDocument(Vec<ParsedElement>);

impl ConundrumDocument {
    pub fn parse_input(input: &mut ConundrumInput) -> ConundrumModalResult<Self> {
        // let r: Vec<MarkdownParagraphResult> =
        //     repeat(0..,
        // MarkdownParagraphResult::parse_input_string).parse_next(input)?;
        // Ok(ConundrumDocument(r))
        let res = parse_elements.parse_next(input)?;
        Ok(Self(res))
    }

    pub fn elements(&self) -> Vec<ParsedElement> {
        // let y: Vec<ParsedElement> = self.0.iter().map(|n|
        // n.children.0.clone()).flatten().collect(); y
        self.0.clone()
    }

    pub fn compile_sync(&self, state: &mut ParseState) -> ConundrumModalResult<String> {
        let mut s = String::from("");
        for em in self.elements() {
            let r = em.to_html_js_component(state)?;
            s += r.as_str();
        }
        Ok(s)
    }

    pub fn get_glue(&self, state: &ParseState) -> WebGlueAssetData {
        get_glue_asset_data(state, &state.contains_modifier(&ConundrumModifier::Standalone))
    }

    /// ### Requirements for a completely standalone html file
    /// - [x] Renders math straight to html and MML.
    /// - [x] Gathers katex css
    /// - [x] Gathers component javascript
    /// - [ ] Embeds katex fonts
    /// - [ ] Embeds nerd fonts
    pub fn render_standalone(&self, params: &mut ParseState) -> ConundrumModalResult<String> {
        let glue = self.get_glue(params);
        let compiled = self.compile_sync(params)?;
        let templ = StandaloneTemplate::new(get_title_group(params.data.content.clone(),
                                                            params.modifiers.clone(),
                                                            params.compile_target.clone()).map(|n| n.title)
                                                                                          .ok(),
                                            compiled,
                                            glue.js,
                                            glue.css,
                                            params.ui_params.clone());
        let rendered_standalone = templ.render().map_err(|_| {

    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })?;
        Ok(rendered_standalone)
    }

    pub fn render_app_embedded(&self, params: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}
