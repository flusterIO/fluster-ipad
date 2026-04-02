use crate::{
    output::{
        general::component_constants::auto_inserted_component_name::AutoInsertedComponentName,
        parsing_result::{
            ai_serialization_request::AiSerializationRequestPhase1, mdx_parsing_result::MdxParsingResult,
        },
    },
    parsers::markdown::code_block::ParsedCodeBlock,
};

pub fn get_ai_parsing_request_phase_1_content(block: &ParsedCodeBlock, res: &mut MdxParsingResult) -> String {
    res.ai_secondary_parse_requests.push(AiSerializationRequestPhase1 { parsing_result: block.clone() });
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
