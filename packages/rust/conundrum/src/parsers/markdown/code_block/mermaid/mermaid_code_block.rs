use mmdflux::RenderConfig;
use serde::{Deserialize, Serialize};

use crate::parsers::{
    conundrum::logic::number::conundrum_float::ConundrumFloat,
    markdown::code_block::mermaid::{
        mermaid_layout_engine::MermaidLayoutEngine, mermaid_route_style::MermaidRouteStyle, mermaid_theme::MermaidTheme,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MermaidCodeBlock {
    pub content: String,
    pub scale: Option<ConundrumFloat>,
    pub padding: Option<ConundrumFloat>,
    pub node_padding_y: Option<ConundrumFloat>,
    pub node_padding_x: Option<ConundrumFloat>,
    pub route_style: Option<MermaidRouteStyle>,
    pub layout: Option<MermaidLayoutEngine>,
    pub theme: MermaidTheme,
}

impl MermaidCodeBlock {
    fn get_mmd_render_config(&self) -> RenderConfig {
        RenderConfig { layout: Default::default(),
                       layout_engine: self.layout.as_ref().cloned().map(|l| l.to_mmdflux()),
                       cluster_ranksep: Default::default(),
                       padding: Default::default(),
                       text_color_mode: Default::default(),
                       svg_scale: self.scale.map(|s| s.0),
                       edge_preset: Default::default(),
                       routing_style: self.route_style.as_ref().cloned().map(|x| x.to_mmdflux()),
                       curve: Default::default(),
                       edge_radius: Default::default(),
                       svg_diagram_padding: self.padding.map(|x| x.0),
                       svg_node_padding_x: self.node_padding_x.map(|x| x.0),
                       svg_node_padding_y: self.node_padding_y.map(|x| x.0),
                       svg_theme: Default::default(),
                       font_metrics_profile: Default::default(),
                       graph_text_style: Default::default(),
                       show_ids: Default::default(),
                       geometry_level: Default::default(),
                       path_simplification: Default::default() }
    }
}
