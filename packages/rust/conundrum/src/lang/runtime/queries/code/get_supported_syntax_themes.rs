use syntect::highlighting::ThemeSet;

pub fn get_supported_syntax_themes() -> Vec<String> {
    let theme_sets = ThemeSet::load_defaults();
    theme_sets.themes.keys().cloned().collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn gets_supported_syntax_themes() {
        let res = get_supported_syntax_themes();
        assert!(!res.is_empty(), "Returns valid syntax themes.");
    }
}
