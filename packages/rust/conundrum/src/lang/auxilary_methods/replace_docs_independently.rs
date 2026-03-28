use crate::parsers::conundrum::docs::ParsedInspectionRequest;

pub fn replace_docs_independently(content: &str) -> (String, bool) {
    ParsedInspectionRequest::replace_docs_independently(content)
}
