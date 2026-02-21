pub fn split_biblatex_to_raw_strings(file_content: &str) -> Vec<String> {
    let mut entries: Vec<String> = Vec::new();
    let mut current_entry = String::new();
    let mut brace_depth = 0;
    let mut inside_entry = false;

    for c in file_content.chars() {
        // Look for the start of a new entry
        if !inside_entry {
            if c == '@' {
                inside_entry = true;
                current_entry.push(c);
            }
        } else {
            current_entry.push(c);

            // Track nested braces
            if c == '{' {
                brace_depth += 1;
            } else if c == '}' {
                brace_depth -= 1;

                // When depth returns to 0, the entry is completely closed
                if brace_depth == 0 {
                    entries.push(current_entry.trim().to_string());
                    current_entry.clear();
                    inside_entry = false;
                }
            }
        }
    }

    entries
}
