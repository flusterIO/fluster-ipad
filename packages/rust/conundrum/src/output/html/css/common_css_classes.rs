use crate::lang::constants::ui_constants::MAX_DOCUMENT_WIDTH;

pub fn max_width_class() -> String {
    format!("max-w-[min({}px,100%)]", MAX_DOCUMENT_WIDTH)
}
