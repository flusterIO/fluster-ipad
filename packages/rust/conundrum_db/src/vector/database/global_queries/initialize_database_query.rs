use askama::Template;
use strum::IntoEnumIterator;

use crate::{
    schemas_string,
    vector::{
        database::db_traits::pure_model_static::PureModelStaticMethods,
        models::{
            academic::question::flashcard::pure_flashcard::PureFlashcard,
            taggables::{subject::Subject, tag::Tag, topic::Topic},
        },
    },
};

#[derive(Template)]
#[template(ext = "jinja", escape = "none", path = "schemas/pre_release.sql")]
pub struct InitializeDatabaseQuery {
    schema_string_from_macro: String,
}

impl Default for InitializeDatabaseQuery {
    fn default() -> Self {
        let s = schemas_string!(Tag, Topic, Subject, PureFlashcard<String>);
        Self { schema_string_from_macro: s }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generates_initialize_database_query() {
        let query = InitializeDatabaseQuery::default();
        let rendered = query.render()
                            .inspect_err(|e| {
                                println!("Error: {}", e);
                            })
                            .expect("You better render");
        println!("Rendered: {}", rendered);
        // assert_eq!(result, 4);
    }
}
