use mmdflux::EngineAlgorithmId;
use serde::{Deserialize, Serialize};

#[typeshare::typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, strum_macros::Display)]
pub enum MermaidLayoutEngine {
    #[serde(rename = "flux-layered")]
    #[strum(to_string = "flux-layered")]
    FluxLayered,
    #[serde(rename = "mermaid-layered")]
    #[strum(to_string = "mermaid-layered")]
    MermaidLayered,
    #[serde(rename = "elk-layered")]
    #[strum(to_string = "elk-layered")]
    ElkLayered,
    #[serde(rename = "elk-mrtree")]
    #[strum(to_string = "elk-mrtree")]
    ELkTree,
}

impl MermaidLayoutEngine {
    pub fn to_mmdflux(&self) -> EngineAlgorithmId {
        match self {
            Self::MermaidLayered => EngineAlgorithmId::MERMAID_LAYERED,
            Self::FluxLayered => EngineAlgorithmId::FLUX_LAYERED,
            Self::ELkTree => EngineAlgorithmId::ELK_MRTREE,
            Self::ElkLayered => EngineAlgorithmId::ELK_MRTREE,
        }
    }
}
