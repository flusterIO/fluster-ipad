use mmdflux::format::RoutingStyle;
use serde::{Deserialize, Serialize};

#[typeshare::typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, strum_macros::Display)]
pub enum MermaidRouteStyle {
    #[serde(rename = "orthogonal")]
    #[strum(to_string = "orthogonal")]
    Orthogonal,
    #[serde(rename = "direct")]
    #[strum(to_string = "direct")]
    Direct,
    #[serde(rename = "polyline")]
    #[strum(to_string = "polyline")]
    Polyline,
}

impl MermaidRouteStyle {
    pub fn to_mmdflux(&self) -> RoutingStyle {
        match self {
            Self::Orthogonal => RoutingStyle::Orthogonal,
            Self::Polyline => RoutingStyle::Polyline,
            Self::Direct => RoutingStyle::Direct,
        }
    }
}
