use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector};

use crate::{
    site::content::novels::NovelRaw,
    utils::{http::fetch_url, time::current_stamp},
};

struct Selectors {
    title: Selector,
    slug: Selector,
    thumbnail: Selector,
    description: Selector,
    preview: Selector,
}

static SELECTORS: Lazy<Selectors> = Lazy::new(|| Selectors {
    title: Selector::parse(r#"p[style*="bold"]"#).unwrap(),
    slug: Selector::parse(".series-title a").unwrap(),
    thumbnail: Selector::parse(r#"div[data-bg]"#).unwrap(),
    description: Selector::parse(r#"div[style]"#).unwrap(),
    preview: Selector::parse(".thumb-section-flow main.row .thumb-item-flow .ln-tooltip").unwrap(),
});

fn get_item(tooltip: &ElementRef, preview: &ElementRef) -> NovelRaw {
    NovelRaw {
        id: parse_id(tooltip),
        title: parse_title(tooltip),
        slug: parse_slug(preview),
        thumbnail: parse_thumbnail(preview),
        description: parse_description(tooltip),
        author_id: Some(0),
        artist_id: Some(0),
        created_at: current_stamp() as i64,
        updated_at: current_stamp() as i64,
    }
}

pub async fn parse_items(html: &str) -> Vec<NovelRaw> {
    let raw = Html::parse_document(html);
    let mut result: Vec<NovelRaw> = Vec::new();

    for (_i, preview) in raw.select(&SELECTORS.preview).enumerate() {
        // attr format: data-tooltip-content="#series_15056"
        let tooltip = preview
            .attr("data-tooltip-content")
            .and_then(|id| Selector::parse(id).ok())
            .and_then(|selector| raw.select(&selector).next());

        if let Some(tooltip) = tooltip {
            result.push(get_item(&tooltip, &preview));
        }
    }

    result
}

fn parse_id(tooltip: &ElementRef) -> i64 {
    tooltip
        .attr("id")
        .unwrap()
        .trim()
        .strip_prefix("series_")
        .unwrap()
        .parse::<i64>()
        .unwrap()
}

fn parse_title(tooltip: &ElementRef) -> String {
    tooltip
        .select(&SELECTORS.title)
        .next()
        .unwrap()
        .inner_html()
        .trim()
        .to_string()
}

fn parse_slug(preview: &ElementRef) -> String {
    preview
        .parent()
        .and_then(ElementRef::wrap)
        .unwrap()
        .select(&SELECTORS.slug)
        .next()
        .unwrap()
        .attr("href")
        .unwrap()
        .to_string()
}

fn parse_thumbnail(preview: &ElementRef) -> Option<String> {
    Some(
        preview
            .select(&SELECTORS.thumbnail)
            .next()
            .unwrap()
            .attr("data-bg")
            .unwrap()
            .trim()
            .to_string(),
    )
}

fn parse_description(tooltip: &ElementRef) -> Option<String> {
    Some(
        tooltip
            .select(&SELECTORS.description)
            .next()
            .unwrap()
            .last_child()
            .unwrap()
            .value()
            .as_text()
            .unwrap()
            .trim()
            .to_string(),
    )
}
