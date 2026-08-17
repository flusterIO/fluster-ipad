use conundrum::lifted_models::primitives::db_id::DatabaseId;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct SnippetApplicationSettings {
    pub snippet_id: DatabaseId,
    /// An optional 'trigger' that can be used by applications that choose to
    /// support the Conundrum ecosystem.
    pub trigger: Option<String>,
    /// Treat this snippet as an 'auto-snippet', automatically expanding it when
    /// the trigger is reached via a strict equality check.
    pub auto_trigger: bool,
    /// Treat the `trigger` field as a regular expression and expand when that
    /// trigger is matched.
    pub regex_trigger: bool,
}
