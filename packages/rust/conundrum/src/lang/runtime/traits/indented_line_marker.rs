pub trait IndentedLineMarker {
    fn string_representation(&self) -> String;
    fn should_continue_indented_line_iter(&self) -> bool;
}
