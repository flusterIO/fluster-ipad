pub fn format_javascript_string(val: &str) -> String {
    val.replace("\\", "\\\\").replace("\"", "\\\"")
}
