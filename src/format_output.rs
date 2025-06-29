pub fn format_output(lines: Option<usize>, words: Option<usize>, bytes: Option<usize>, file_name: Option<&str>) -> String {
    let line_str = lines.map_or(String::new(), |v| format!("{:>8}", v));
    let word_str = words.map_or(String::new(), |v| format!("{:>8}", v));
    let byte_str = bytes.map_or(String::new(), |v| format!("{:>8}", v));
    let name_str = file_name.unwrap_or("");

    format!("{}{}{} {}", line_str, word_str, byte_str, name_str)
}
