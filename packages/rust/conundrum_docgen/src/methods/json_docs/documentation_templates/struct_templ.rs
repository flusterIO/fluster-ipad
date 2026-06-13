use core::panic;

use rustdoc_types::{Crate, GenericArg, GenericArgs, Item, ItemEnum, Struct, Type};

use crate::methods::json_docs::documentation_templates::{
    struct_field::StructField, struct_general_field::StructGeneralField,
};

#[derive(askama::Template, Clone)]
#[template(path = "markdown/documentation/from_rust/struct/struct.txt")]
pub struct StructItemTemplate {
    pub item: Item,
    pub inner: Struct,
    pub crate_docs: Crate,
}

impl StructItemTemplate {
    pub fn get_fields(&self, crate_docs: Crate) -> Vec<StructField> {
        let fields = match self.clone().inner.kind {
            rustdoc_types::StructKind::Plain { fields,
                                               has_stripped_fields, } => {
                let mut items: Vec<StructField> = Vec::new();
                for f in fields {
                    let field_data =
                        crate_docs.index.get(&f).expect("Could not find a value with a documentation key.");
                    if let Some(data_item) = match &field_data.inner {
                        ItemEnum::StructField(s) => match s {
                            Type::Primitive(p) => {
                                panic!("This documentation route for the primitive type has not been implemented.")
                            }
                            Type::ResolvedPath(p) => {
                                if p.path == "Option" {
                                    let option_args = *p.args.as_ref().cloned().expect("Documentable options must have some associated option type... I think?");
                                    match option_args.clone() {
                                        GenericArgs::AngleBracketed { args,
                                                                      constraints, } => {
                                            match args.first()
                                                      .expect("Found an option with zero arguments it looks like?")
                                            {
                                                GenericArg::Type(t) => match t {
                                                    Type::ResolvedPath(p) => {
                                                        let dtype = crate_docs.index.get(&p.id).expect("Failed to lookup an Option nested item.");
                                                        println!("Dtype: {:#?}", dtype);
                                                        None
                                                    }
                                                    _ => {
                                                        panic!("Not all arms of the option arguments match statement are covered. Handle that as documentation is slowly moved to Rust.")
                                                    }
                                                },
                                                _ => {
                                                    panic!("Not all arms of the option arguments match statement are covered. Handle that as documentation is slowly moved to Rust.")
                                                }
                                            }
                                        }
                                        _ => {
                                            panic!("Not all arms of the option_args match statement are covered. Handle this as the documentation is gradually moved to Rust.")
                                        }
                                    }
                                } else {
                                    None
                                }
                            }
                            _ => {
                                panic!("This documentation route for the primitive type has not been implemented. {:#?}",
                                       s.clone())
                            }
                        },
                        _ => None,
                    } {
                        items.push(data_item);
                    }
                }
                items
            }
            _ => {
                panic!("The get_fields method for the StructItemTemplate is incomplete. Please handle the missing variants to parse this struct documentation.")
            }
        };

        return fields;
    }
}
