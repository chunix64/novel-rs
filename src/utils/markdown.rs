use scraper::ElementRef;

pub fn image_to_md(element: &ElementRef) -> String {
    let src = element.attr("src").unwrap().trim().to_string();
    let alt = element.attr("alt").unwrap().trim().to_string();
    format!("![{}]({})", alt, src)
}

pub fn em_to_md(element: &ElementRef) -> String {
    let text = element.inner_html();
    format!("_{}_", text)
}
