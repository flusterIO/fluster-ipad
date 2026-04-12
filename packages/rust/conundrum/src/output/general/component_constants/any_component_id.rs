use std::{fmt::Display, str::FromStr};

use winnow::error::ErrMode;

use crate::{
    lang::runtime::state::{
        conundrum_error::ConundrumError,
        conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
    },
    output::general::component_constants::{
        auto_inserted_component_name::AutoInsertedComponentName, component_names::EmbeddableComponentName,
        documentation_component_name::DocumentationComponentName,
    },
};

pub enum AnyComponentName {
    UserEmbedded(EmbeddableComponentName),
    Docs(DocumentationComponentName),
    AutoInserted(AutoInsertedComponentName),
}

impl AnyComponentName {
    pub fn get_component_name(name: &str) -> ConundrumModalResult<AnyComponentName> {
        if let Ok(user_embedded) = EmbeddableComponentName::from_str(name) {
            Ok(AnyComponentName::UserEmbedded(user_embedded))
        } else if let Ok(docs) = DocumentationComponentName::from_str(name) {
            Ok(AnyComponentName::Docs(docs))
        } else if let Ok(auto_inserted) = AutoInsertedComponentName::from_str(name) {
            Ok(AnyComponentName::AutoInserted(auto_inserted))
        } else {
            Err(
            ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Fail to find component", format!("Conundrum attempted to lookup a component `{}` that does not exist. See the `Components??` documentation for a list of available components.", name).as_str())))
        )
        }
    }
}

impl Display for AnyComponentName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AnyComponentName::UserEmbedded(u) => u.to_string(),
            AnyComponentName::Docs(u) => u.to_string(),
            AnyComponentName::AutoInserted(u) => u.to_string(),
        })
    }
}
