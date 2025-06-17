use scraper::ElementRef;

pub fn image_to_md(element: &ElementRef) -> String {
    let src = element.attr("src").map(str::trim).unwrap_or_default();
    let alt = element.attr("alt").map(str::trim).unwrap_or_default();
    format!("![{}]({})", alt, src)
}

pub fn em_to_md(element: &ElementRef) -> String {
    let text = element.inner_html();
    format!("_{}_", text)
}
