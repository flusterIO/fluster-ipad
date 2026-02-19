uniffi::setup_scaffolding!();

// ... classes with methods ...
#[derive(uniffi::Object)]
pub struct BibEntryData {
    body: String,
}

// Return a json serializable string. Use Rust to read values directly from the bib entry if needed
// instead of parsing it into a map because of the awkward, largely unknown shape.
#[uniffi::export]
pub fn parse_bib_entry(bib_entry_body: String) {}

#[uniffi::export]
impl BibEntryData {
    // Constructors need to be annotated as such
    #[uniffi::constructor]
    pub fn new(body: String) -> Self {
        Self { body }
    }

    pub fn parse(&self) -> String {
        format!("Hello, {}!", self.body)
    }
}

