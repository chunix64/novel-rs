use scraper::{ElementRef, html::Select};

use crate::utils::markdown::{em_to_md, image_to_md};

pub fn elements_to_markdown(elements: Select, whitespace: &str) -> String {
    let mut result: String = String::new();
    for element in elements {
        result.push_str(&element_to_markdown(&element, whitespace));
    }
    result
}

pub fn element_to_markdown(element: &ElementRef, whitespace: &str) -> String {
    let mut result: String = String::new();
    for child_element in element.children() {
        if let Some(child) = ElementRef::wrap(child_element) {
            match child.value().name() {
                "img" => result.push_str(&image_to_md(&child)),
                "em" => result.push_str(&em_to_md(&child)),
                "p" => result.push_str(&element_to_markdown(&child, whitespace)),
                "div" => result.push_str(&element_to_markdown(&child, whitespace)),
                _ => {}
            }
        } else if let Some(text) = child_element.value().as_text() {
            result.push_str(text);
        }

        result.push_str(whitespace);
    }

    // remove last whitespace
    if result.ends_with(whitespace) {
        result.truncate(result.len() - whitespace.len());
    }

    result
}
