use dashmap::DashMap;

use crate::lang::elements::parsed_elements::ParsedElement;

pub struct Scope {
    pub data: DashMap<String, ParsedElement>,
    pub enclosing: Option<Box<Scope>>,
}
