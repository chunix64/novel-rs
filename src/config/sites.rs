use crate::utils::string::to_snake_case;

pub struct Site {
    pub name: String,
    pub alias: String,
    pub db_name: String,
    pub language: String,
}

impl Site {
    fn new(
        name: impl Into<String>,
        alias: Option<String>,
        db_name: Option<String>,
        language: impl Into<String>,
    ) -> Self {
        let name: String = name.into();
        let alias = alias.unwrap_or_else(|| to_snake_case(&name));
        let db_name = db_name.unwrap_or_else(|| alias.clone());
        let language_str = language.into();
        let language = if language_str.trim().is_empty() {
            "und".to_string() // und is undetermined language (unknown)
        } else {
            language_str
        };

        Self {
            name,
            alias,
            db_name,
            language,
        }
    }
}

pub fn get_sites() -> Vec<Site> {
    vec![Site::new("docln", None, None, "vi")]
}
