pub fn format_embedded_object_property(item: String) -> String {
    format!("{{{}}}", item)
}

pub fn javascript_null_prop() -> String {
    "{null}".to_string()
}
