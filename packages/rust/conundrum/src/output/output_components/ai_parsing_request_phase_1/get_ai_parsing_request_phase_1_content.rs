use fluster_core_utilities::core_types::component_constants::auto_inserted_component_name::AutoInsertedComponentName;

use crate::{
    lang::elements::parsed_code_block::ParsedCodeBlock,
    output::parsing_result::{
        ai_serialization_request::AiSerializationRequestPhase1, mdx_parsing_result::MdxParsingResult,
    },
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
