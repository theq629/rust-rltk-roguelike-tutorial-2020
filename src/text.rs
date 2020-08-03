pub fn capitalize<S: ToString>(string: &S) -> String {
    let string = string.to_string();
    let mut chars = string.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str()
    }
}
