use crate::{
    output::general::component_constants::auto_inserted_component_name::AutoInsertedComponentName,
    parsers::markdown::code_block::code_block_model::GeneralCodeBlock,
};

pub fn get_ai_parsing_request_phase_1_content(block: &GeneralCodeBlock) -> String {
    format!(
            r#"
<{} res={}>
{}
</{}>
"#,
            AutoInsertedComponentName::FlusterAiParsePendingContainer,
            serde_json::to_string(block).map(|x| format!("{{{}}}", x)).unwrap_or("{null}".to_string()),
            block.content,
            AutoInsertedComponentName::FlusterAiParsePendingContainer
    )
}
