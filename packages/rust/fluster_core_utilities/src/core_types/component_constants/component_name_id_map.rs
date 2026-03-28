use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::core_types::component_constants::{
    component_ids::EmbeddableComponentId, component_names::EmbeddableComponentName,
};

lazy_static! {
    pub static ref COMPONENT_NAME_ID_MAP: HashMap<EmbeddableComponentName, EmbeddableComponentId> = {
        let mut map = HashMap::new();
        map.insert(EmbeddableComponentName::Card, EmbeddableComponentId::Card);
        map.insert(EmbeddableComponentName::Grid, EmbeddableComponentId::Grid);
        map.insert(EmbeddableComponentName::Ul, EmbeddableComponentId::Ul);
        map.insert(EmbeddableComponentName::AINoteSummary, EmbeddableComponentId::AINoteSummary);
        map.insert(EmbeddableComponentName::Underline, EmbeddableComponentId::Ul);
        map.insert(EmbeddableComponentName::Hl, EmbeddableComponentId::Hl);
        map.insert(EmbeddableComponentName::Hint, EmbeddableComponentId::Hint);
        map.insert(EmbeddableComponentName::Highlight, EmbeddableComponentId::Hl);
        map.insert(EmbeddableComponentName::Admonition, EmbeddableComponentId::Admonition);
        map.insert(EmbeddableComponentName::HrWithChildren, EmbeddableComponentId::HrWithChildren);
        map.insert(EmbeddableComponentName::UtlityContainer, EmbeddableComponentId::UtlityContainer);
        map
    };
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn component_name_id_map_is_exhaustive() {
        for component_name in EmbeddableComponentName::iter() {
            let value = COMPONENT_NAME_ID_MAP.get(&component_name);
            assert!(value.is_some(),
                    "The {} component could not be found in the COMPONENT_NAME_ID_MAP map.",
                    component_name);
        }
    }
}
