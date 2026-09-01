use conundrum::lang::runtime::run_conundrum::ParseConundrumOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, specta::Type)]
pub struct IpynbParseParameters {
    pub cdrm: Option<ParseConundrumOptions>,
}
