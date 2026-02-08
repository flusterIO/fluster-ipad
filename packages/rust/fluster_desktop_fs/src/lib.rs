uniffi::setup_scaffolding!();


// You can annotate items with uniffi macros to make them available in your swift package.
// You can export functions...
#[uniffi::export]
pub fn add(a: u64, b: u64) -> u64 {
    a + b
}

// ... data structs without methods ...
#[derive(uniffi::Record)]
pub struct Example {
    pub items: Vec<String>,
    pub value: Option<f64>,
}

// ... classes with methods ...
#[derive(uniffi::Object)]
pub struct Greeter {
    name: String,
}

#[uniffi::export]
impl Greeter {
    // Constructors need to be annotated as such
    #[uniffi::constructor]
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

// ... and much more! For more information about uniffi macros, read the UniFFI book: https://mozilla.github.io/uniffi-rs/proc_macro/index.html