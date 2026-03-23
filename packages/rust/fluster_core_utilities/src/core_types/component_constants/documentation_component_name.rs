use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use typeshare::typeshare;

#[typeshare]
#[derive(Display, EnumIter, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentationComponentName {
    #[serde(rename = "InContentDocumentationContainer")]
    #[strum(to_string = "InContentDocumentationContainer")]
    InContentDocumentationContainer,
    #[serde(rename = "InContentDocsEmphasisTypeList")]
    #[strum(to_string = "InContentDocsEmphasisTypeList")]
    InContentDocsEmphasisTypeList,
    #[serde(rename = "InContentDocsHighlightDemo")]
    #[strum(to_string = "InContentDocsHighlightDemo")]
    InContentDocsHighlightDemo,
    #[serde(rename = "InContentDocsUnderlineDemo")]
    #[strum(to_string = "InContentDocsUnderlineDemo")]
    InContentDocsUnderlineDemo,
}
