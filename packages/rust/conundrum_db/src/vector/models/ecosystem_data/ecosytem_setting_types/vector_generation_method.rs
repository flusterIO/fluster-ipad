/// This type should only be applied to structs who's vectors are not critical
/// for the functioning of the app.
#[derive(serde::Serialize,
           serde::Deserialize,
           Clone,
           Debug,
           Default,
           specta::Type,
           fake::Dummy,
           strum_macros::EnumString,
           strum_macros::EnumIter)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum OptionalVectorGenerationMethod {
    LocalOnly,
    #[default]
    LocalAndRemote,
}
