pub fn capitalize(string: &String) -> String {
    let mut chars = string.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str()
    }
}
