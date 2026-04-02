use serde::Serialize;

#[typeshare::typeshare]
#[derive(Serialize, Debug, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum CodeBlockLanguage {
    DefaultLanguage,
    UserProvided(String),
}
