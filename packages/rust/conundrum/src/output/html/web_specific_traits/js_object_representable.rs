use serde::Serialize;
use winnow::error::ErrMode;

use crate::lang::runtime::state::conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult};

pub trait JavascriptObjectRepresentable: Serialize {
    fn to_javascript_object_string(&self) -> ConundrumModalResult<String> {
        serde_json::to_string(&self).map_err(|e| {
                                        eprintln!("Serialization Error: {}", e);
                                        ErrMode::Cut(ConundrumErrorVariant::FailToGenerateString)
                                    })
    }
}
