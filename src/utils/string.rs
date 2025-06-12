pub fn get_valid_file_name(file_name: &str) -> String {
    let separate = "__";
    file_name
        .trim_start_matches("/")
        .trim_start_matches("\\")
        .replace("/", separate)
        .replace("\\", separate)
}
