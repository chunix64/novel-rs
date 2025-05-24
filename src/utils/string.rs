pub fn to_snake_case(text: &str) -> String {
    let mut result: String = String::new();
    for (i, c) in text.trim().chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            if c == ' ' {
                result.push('_');
            } else {
                result.push(c);
            }
        }
    }
    result
}

pub fn get_valid_file_name(file_name: &str) -> String {
    let separate = "__";
    file_name
        .strip_prefix("/")
        .unwrap()
        .replace("/", separate)
        .replace("\\", separate)
}
