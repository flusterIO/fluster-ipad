use mmdflux::{OutputFormat, RenderConfig, render_diagram};
use serde::{Deserialize, Serialize};
use winnow::error::ErrMode;

use crate::{
    lang::runtime::{
        state::conundrum_error_variant::ConundrumErrorVariant,
        traits::{
            fluster_component_result::ConundrumComponentResult, html_js_component_result::HtmlJsComponentResult,
            markdown_component_result::MarkdownComponentResult, plain_text_component_result::PlainTextComponentResult,
        },
    },
    parsers::{
        conundrum::logic::number::conundrum_number::ConundrumNumber,
        markdown::code_block::mermaid::{
            mermaid_layout_engine::MermaidLayoutEngine, mermaid_route_style::MermaidRouteStyle,
            mermaid_theme::MermaidTheme,
        },
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MermaidCodeBlock {
    pub content: String,
    pub scale: Option<ConundrumNumber>,
    pub padding: Option<ConundrumNumber>,
    pub node_padding_y: Option<ConundrumNumber>,
    pub node_padding_x: Option<ConundrumNumber>,
    pub route_style: Option<MermaidRouteStyle>,
    pub layout: Option<MermaidLayoutEngine>,
    pub theme: MermaidTheme,
}

impl ConundrumComponentResult for MermaidCodeBlock {
    fn to_conundrum_component(&self,
                              res: crate::lang::runtime::traits::conundrum_input::ArcState)
                              -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String>
    {
        todo!()
    }
}

impl PlainTextComponentResult for MermaidCodeBlock {
    fn to_plain_text(&self,
                     res: crate::lang::runtime::traits::conundrum_input::ArcState)
                     -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        render_diagram(self.content.as_str(), OutputFormat::Text, &self.get_mmd_render_config()).map_err(|e| {
    ErrMode::Backtrack(ConundrumErrorVariant::MermaidError(e.message))
})
    }
}

impl MarkdownComponentResult for MermaidCodeBlock {
    fn to_markdown(&self,
                   res: crate::lang::runtime::traits::conundrum_input::ArcState)
                   -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        render_diagram(self.content.as_str(), OutputFormat::Text, &self.get_mmd_render_config()).map_err(|e| {
    ErrMode::Backtrack(ConundrumErrorVariant::MermaidError(e.message))
})
    }
}

impl HtmlJsComponentResult for MermaidCodeBlock {
    fn to_html_js_component(&self,
                            _: crate::lang::runtime::traits::conundrum_input::ArcState)
                            -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        let s = render_diagram(self.content.as_str(), OutputFormat::Svg, &self.get_mmd_render_config()).map_err(|e| {
    ErrMode::Backtrack(ConundrumErrorVariant::MermaidError(e.message))
})?;
        return Ok(format!("<div class=\"my-6 [&>svg]:mx-auto\">{}</div>", s));
    }
}

impl MermaidCodeBlock {
    fn get_mmd_render_config(&self) -> RenderConfig {
        RenderConfig { layout: Default::default(),
                       layout_engine: self.layout.as_ref().cloned().map(|l| l.to_mmdflux()),
                       cluster_ranksep: Default::default(),
                       padding: Default::default(),
                       text_color_mode: Default::default(),
                       svg_scale: self.scale.map(|s| s.as_float()),
                       edge_preset: Default::default(),
                       routing_style: self.route_style.as_ref().cloned().map(|x| x.to_mmdflux()),
                       curve: Default::default(),
                       edge_radius: Default::default(),
                       svg_diagram_padding: self.padding.map(|x| x.as_float()),
                       svg_node_padding_x: self.node_padding_x.map(|x| x.as_float()),
                       svg_node_padding_y: self.node_padding_y.map(|x| x.as_float()),
                       svg_theme: Some(self.theme.to_svg_theme_config()),
                       font_metrics_profile: Default::default(),
                       graph_text_style: Default::default(),
                       show_ids: Default::default(),
                       geometry_level: Default::default(),
                       path_simplification: Default::default() }
    }
}
