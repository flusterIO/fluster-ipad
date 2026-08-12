use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;

#[derive(Serialize, Deserialize, Clone, Debug, strum_macros::Display, strum_macros::EnumIter)]
#[strum(serialize_all = "kebab-case")]
#[serde(try_from = "String")]
pub enum ComponentKind {
    UserEmbeddable,
    Documentation,
}

impl TryFrom<String> for ComponentKind {
    type Error = ConundrumErrorVariant;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for k in ComponentKind::iter() {
            if k.to_string() == value {
                return Ok(k);
            }
        }
        Err(ConundrumErrorVariant::SerializationError { entity_name: "ComponentKind".to_string() })
    }
}
