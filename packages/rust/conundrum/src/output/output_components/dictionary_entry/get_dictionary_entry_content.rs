use crate::{
    lang::elements::parsed_code_block::ParsedCodeBlock,
    output::{
        output_components::{
            dictionary_entry::dictionary_entry_props::DictionaryEntryResultData,
            output_utils::format_embedded_object_property,
        },
        parsing_result::{
            dictionary_result::DictionaryEntryResult, mdx_parsing_result::MdxParsingResult,
        },
    },
};

pub fn get_dictionary_content(block: &ParsedCodeBlock, res: &mut MdxParsingResult) -> String {
    let term = block.meta_data.as_deref().unwrap_or("Untitled");
    res.dictionary_entries.push(DictionaryEntryResult {
        label: term.clone().to_string(),
        body: block.content.clone(),
    });
    let component_props = DictionaryEntryResultData {
        label: term.to_string(),
        note_id: res.note_id.clone(),
    };
    format!(
        r#"
<DictionaryEntry entryData={}>
{}
</DictionaryEntry>
"#,
        serde_json::to_string(&component_props)
            .map(format_embedded_object_property)
            .unwrap_or("{null}".to_string()),
        block.content
    )
}
