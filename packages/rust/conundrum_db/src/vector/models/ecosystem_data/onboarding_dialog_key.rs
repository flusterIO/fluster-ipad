use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Serialize, Deserialize, Clone, Debug, strum_macros::Display, EnumIter)]
pub enum OnboardingDialogKey {
    #[serde(rename = "workspace-management-home")]
    #[strum(to_string = "workspace-management-home")]
    WorkspaceManagementHome,
    #[serde(rename = "initial-dialog")]
    #[strum(to_string = "initial-dialog")]
    InitialDialog,
}
