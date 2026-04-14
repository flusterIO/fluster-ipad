use crate::{
    output::{
        output_components::{
            dictionary_entry::dictionary_entry_props::DictionaryEntryResultData,
            output_utils::{format_embedded_object_property, javascript_null_prop},
        },
        parsing_result::{dictionary_result::DictionaryEntryResult, mdx_parsing_result::MdxParsingResult},
    },
    parsers::markdown::code_block::code_block::ParsedCodeBlock,
};

pub fn get_dictionary_content(block: &ParsedCodeBlock, res: &mut MdxParsingResult) -> String {
    let term = block.meta_data.as_deref().unwrap_or("Untitled");
    res.dictionary_entries.push(DictionaryEntryResult { label: term.to_string(),
                                                        body: block.content.clone() });
    let component_props = DictionaryEntryResultData { label: term.to_string(),
                                                      note_id: res.note_id.clone() };
    format!(
            r#"
<DictionaryEntry entryData={}>
{}
</DictionaryEntry>
"#,
            serde_json::to_string(&component_props).map(format_embedded_object_property)
                                                   .unwrap_or(javascript_null_prop()),
            block.content
    )
}
