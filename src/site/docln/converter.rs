use scraper::{ElementRef, html::Select};

use crate::utils::markdown::image_to_md;

pub fn chapter_to_markdown(contents: Select) -> String {
    let mut result: String = String::new();
    for content in contents {
        for child_element in content.children() {
            if let Some(child) = ElementRef::wrap(child_element) {
                match child.value().name() {
                    "img" => result.push_str(&image_to_md(&child)),
                    _ => {}
                }
            } else if let Some(text) = child_element.value().as_text() {
                result.push_str(text);
            }

            result.push_str("\n\n");
        }
    }
    result
}
