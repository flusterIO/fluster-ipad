use syntect::parsing::SyntaxSet;

pub fn get_supported_syntaxes() -> Vec<String> {
    SyntaxSet::load_defaults_newlines().syntaxes().iter().map(|item| item.name.clone()).collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use rssn::prelude::argmin::seq::IndexedRandom;

    use super::*;

    #[test]
    fn gets_supported_syntaxes() {
        let res = get_supported_syntaxes();
        assert!(!res.is_empty(), "Finds valid syntaxes.");
    }
}
