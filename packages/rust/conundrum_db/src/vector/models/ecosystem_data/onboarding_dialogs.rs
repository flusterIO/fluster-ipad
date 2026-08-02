use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::vector::models::ecosystem_data::onboarding_dialog_key::OnboardingDialogKey;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OnboardingDialogs(HashMap<OnboardingDialogKey, bool>);

impl Default for OnboardingDialogs {
    fn default() -> Self {
        let mut hm = HashMap::new();
        for item in OnboardingDialogKey::iter() {
            hm.insert(item, false);
        }
        Self(hm)
    }
}
