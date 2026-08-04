#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub enum TerminalOrHtml {
    Terminal,
    Html,
}
