use crate::{
    lang::runtime::traits::conundrum_input::ArcState,
    output::output_components::{
        dictionary_entry::dictionary_entry_props::DictionaryEntryResultData,
        output_utils::{format_embedded_object_property, javascript_null_prop},
    },
    parsers::markdown::code_block::code_block_model::GeneralCodeBlock,
};

pub fn get_dictionary_content(block: &GeneralCodeBlock, res: ArcState) -> String {
    let state = res.read_arc();
    let term = block.meta_data.as_deref().unwrap_or("Untitled");
    let component_props = DictionaryEntryResultData { label: term.to_string(),
                                                      note_id: state.data.note_id.clone() };
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
