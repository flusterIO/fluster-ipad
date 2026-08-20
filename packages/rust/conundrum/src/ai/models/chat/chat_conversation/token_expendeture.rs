#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct TokenExpendeture {
    pub total: u32,
    pub incoming: u32,
    pub outgoing: u32,
}
