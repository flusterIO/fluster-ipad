use serde::Serialize;

#[typeshare::typeshare]
#[derive(Serialize, Debug)]
#[serde(tag = "tag", content = "content")]
pub enum CodeBlockLanguage {
    DefaultLanguage,
    UserProvided(String),
}
