use askama::Template;
use strum::IntoEnumIterator;

#[derive(Template)]
#[template(ext = "jinja", escape = "none", path = "queries/create_tables.sql")]
pub struct InitializeDatabaseQuery {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generates_initialize_database_query() {
        let query = InitializeDatabaseQuery {};
        let rendered = query.render()
                            .inspect_err(|e| {
                                println!("Error: {}", e);
                            })
                            .expect("Motherfucker you better render");
        println!("Rendered: {}", rendered);
        // assert_eq!(result, 4);
    }
}
