use serde::Serialize;
use typeshare::typeshare;
use uniffi::Enum;

#[typeshare]
#[derive(Serialize, strum_macros::Display, Enum)]
pub enum ConundrumStateActions {
    #[serde(rename = "set-conundrum-error")]
    #[strum(to_string = "set-conundrum-error")]
    SetConundrumErrors,
}
