use serde::Serialize;

use crate::lang::lib::ui::ui_types::children::Children;

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct OrderedListItem {
    pub content: Children,
}
