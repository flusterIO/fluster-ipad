use crate::lang::elements::parsed_elements::ParsedElement;

/// Collects only the plain-text leaves of the parsed content for easy
/// assertions.
pub fn get_children_content_text(children: &[ParsedElement]) -> String {
    children.iter()
            .map(|e| match e {
                ParsedElement::Text(s) => s.clone(),
                _ => String::new(),
            })
            .collect::<Vec<_>>()
            .join("")
}
