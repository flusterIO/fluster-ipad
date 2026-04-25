use crate::parsers::conundrum::docs::docs_model::ParsedInspectionRequest;

pub fn replace_docs_independently(content: &str) -> (String, bool) {
    ParsedInspectionRequest::replace_docs_independently(content)
}
