use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
#[surreal(untagged)]
pub enum TaggableVariant {
    #[surreal(value = "tag")]
    Tag,
    #[surreal(value = "topic")]
    Topic,
    #[surreal(value = "subject")]
    Subject,
}
