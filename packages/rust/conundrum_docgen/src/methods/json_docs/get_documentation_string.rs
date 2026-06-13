use askama::Template;
use rustdoc_types::{Crate, Item, ItemEnum, Struct};

use crate::{
    errors::DocGenResult,
    methods::json_docs::documentation_templates::{
        rust_doc_template::RustDocTemplate, struct_templ::StructItemTemplate,
    },
};
/// The documentation of the item with the
/// userfacing flags removed.
pub fn get_documentation_string(item: Item, crate_docs: Crate) -> DocGenResult<String> {
    let templ = match item.clone().inner {
        ItemEnum::Struct(inner) => RustDocTemplate::Struct(StructItemTemplate { item: item.clone(),
                                                                                inner,
                                                                                crate_docs: crate_docs.clone() }),
        _ => {
            panic!("Conundrum docgen attempted to generate documentation for a type that is not yet enabled: {:?}",
                   item.inner.clone())
        }
    };
    templ.render()
}
