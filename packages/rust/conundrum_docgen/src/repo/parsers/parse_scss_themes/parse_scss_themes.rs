pub fn parse_scss_themes() -> DocgenError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_scss_themes_without_throwing_an_error() {
        let root = crate::workspace_utils::geget_workspace_root_duplicate::get_workspace_root();
        // assert_eq!(result, 4);
    }
}
