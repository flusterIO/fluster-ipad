#[macro_export]
macro_rules! execute_tool_by_name {
    ( $name:literal, $($tool:ident),+ ) => {{
    $(
    if $tool::name() == $name {
        $tool::execute()
    }
    )*
    }};
}
