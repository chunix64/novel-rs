use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector, selectable::Selectable};
use tracing::{error, trace};

use crate::{
    site::content::novels::{ChapterMeta, ChapterRaw, NovelEnrich, NovelRaw},
    utils::time::current_timestamp,
};

use crate::utils::html::{element_to_markdown, elements_to_markdown};

struct Selectors {
    novel_title: Selector,
    novel_slug: Selector,
    novel_thumbnail: Selector,
    novel_description: Selector,
    novel_previews: Selector,
    novel_max_page: Selector,
    chapters: Selector,
    chapter_contents: Selector,
    enrich_info_box: Selector,
    enrich_description: Selector,
    enrich_author: Selector,
    enrich_artist: Selector,
    enrich_tag: Selector,
}

static SELECTORS: Lazy<Selectors> = Lazy::new(|| Selectors {
    novel_title: Selector::parse(r#"p[style*="bold"]"#).unwrap(),
    novel_slug: Selector::parse(".series-title a").unwrap(),
    novel_thumbnail: Selector::parse(r#"div[data-bg]"#).unwrap(),
    novel_description: Selector::parse(r#"div[style]"#).unwrap(),
    novel_previews: Selector::parse(".thumb-section-flow main.row .thumb-item-flow .ln-tooltip")
        .unwrap(),
    novel_max_page: Selector::parse("a.paging_item.paging_prevnext.next").unwrap(),
    chapters: Selector::parse(".chapter-name a").unwrap(),
    chapter_contents: Selector::parse("#chapter-content p").unwrap(),
    enrich_info_box: Selector::parse(".feature-section.at-series.clear").unwrap(),
    enrich_description: Selector::parse(".summary-content").unwrap(),
    enrich_author: Selector::parse(r#".info-value a[href*="/tac-gia/"]"#).unwrap(),
    enrich_artist: Selector::parse(r#".info-value a[href*="/hoa-si/"]"#).unwrap(),
    enrich_tag: Selector::parse(".series-gernes a").unwrap(),
});

// Main function
pub fn parse_novel_max_page(html: &str) -> i64 {
    let document = Html::parse_document(html);
    let max_page = match document.select(&SELECTORS.novel_max_page).next() {
        Some(max_page_element) => {
            let max_page_href = max_page_element.attr("href").unwrap_or_default();
            max_page_href
                .split("page=")
                .nth(1)
                .map(|x| x.trim())
                .unwrap_or_default()
        }
        None => "",
    };

    match max_page.parse::<i64>() {
        Ok(max_page) => max_page,
        Err(error) => {
            error!(target = %"provider", fallback = 1, ?error, "Failed to parse max_page, use fallback = 1");
            1
        }
    }
}

pub fn parse_novels(html: &str) -> Vec<NovelRaw> {
    let raw = Html::parse_document(html);
    let mut result: Vec<NovelRaw> = Vec::new();

    for preview in raw.select(&SELECTORS.novel_previews) {
        // attr format: data-tooltip-content="#series_15056"
        let tooltip = preview
            .attr("data-tooltip-content")
            .and_then(|id| Selector::parse(id).ok())
            .and_then(|selector| raw.select(&selector).next());

        if let Some(tooltip) = tooltip {
            let novel = get_novel(&tooltip, &preview);
            if let Some(novel) = novel {
                result.push(novel);
            }
        }
    }

    result
}

pub fn parse_chapters_list(html: &str) -> Vec<ChapterMeta> {
    let raw = Html::parse_document(html);
    let mut result: Vec<ChapterMeta> = Vec::new();

    for chapter in raw.select(&SELECTORS.chapters) {
        match parse_chapter_meta(&chapter) {
            Some(chapter) => result.push(chapter),
            None => continue,
        };
    }

    result
}

pub fn parse_novel_enrich(html: &str) -> Option<NovelEnrich> {
    let document = Html::parse_document(html);
    let info_box = match document.select(&SELECTORS.enrich_info_box).next() {
        Some(info_box) => info_box,
        None => {
            return None;
        }
    };
    let separate = "; ";
    Some(NovelEnrich {
        description: parse_description_enrich(&info_box),
        authors: parse_authors(&info_box, separate),
        artists: parse_artists(&info_box, separate),
        tags: parse_tags(&info_box),
        updated_at: current_timestamp() as i64,
    })
}

pub fn parse_chapter(
    chapter_meta: &ChapterMeta,
    index: i64,
    novel_id: i64,
    content: String,
) -> ChapterRaw {
    ChapterRaw {
        title: chapter_meta.title.clone(),
        slug: chapter_meta.slug.clone(),
        novel_id,
        created_at: current_timestamp() as i64,
        updated_at: current_timestamp() as i64,
        content,
        chapter_number: Some(index),
    }
}

// Helpers
fn parse_attribute(element: &ElementRef, attribute: &str) -> Option<String> {
    Some(element.attr(attribute)?.trim().to_string())
}

// Content Helpers
pub fn parse_chapter_content(html: &str) -> String {
    let raw = Html::parse_document(html);
    let chapter_contents = raw.select(&SELECTORS.chapter_contents);
    elements_to_markdown(chapter_contents, "\n\n")
}

fn parse_chapter_meta(chapter: &ElementRef) -> Option<ChapterMeta> {
    let slug = match parse_chapter_slug(chapter) {
        Some(slug) => {
            trace!(target = %"provider", %slug, "Parsed slug");
            slug
        }
        None => {
            error!(target = %"provider", "Failed to parsed slug");
            return None;
        }
    };

    Some(ChapterMeta {
        title: parse_chapter_title(chapter).unwrap_or_else(|| "Unknown title".to_string()),
        slug,
    })
}

fn parse_chapter_title(chapter: &ElementRef) -> Option<String> {
    parse_attribute(chapter, "title")
}

fn parse_chapter_slug(chapter: &ElementRef) -> Option<String> {
    parse_attribute(chapter, "href")
}

// Item Helpers
fn get_novel(tooltip: &ElementRef, preview: &ElementRef) -> Option<NovelRaw> {
    Some(NovelRaw {
        id: parse_novel_id(tooltip).unwrap_or(-1),
        title: parse_novel_title(tooltip).unwrap_or_else(|| "Unknown title".to_string()),
        slug: parse_novel_slug(preview)?,
        thumbnail: parse_novel_thumbnail(preview),
        description: parse_novel_description(tooltip),
        authors: Vec::new(),
        artists: Vec::new(),
        tags: Vec::new(),
        created_at: current_timestamp() as i64,
        updated_at: current_timestamp() as i64,
    })
}

fn parse_novel_id(tooltip: &ElementRef) -> Option<i64> {
    parse_attribute(tooltip, "id")?
        .strip_prefix("series_")?
        .parse::<i64>()
        .ok()
}

fn parse_novel_title(tooltip: &ElementRef) -> Option<String> {
    let novel_title_element = tooltip.select(&SELECTORS.novel_title).next()?;

    Some(novel_title_element.inner_html().trim().to_string())
}

fn parse_novel_slug(preview: &ElementRef) -> Option<String> {
    let novel_slug_parent_element = preview.parent().and_then(ElementRef::wrap)?;
    let novel_slug_element = novel_slug_parent_element
        .select(&SELECTORS.novel_slug)
        .next()?;
    parse_attribute(&novel_slug_element, "href")
}

fn parse_novel_thumbnail(preview: &ElementRef) -> Option<String> {
    parse_attribute(
        &preview.select(&SELECTORS.novel_thumbnail).next()?,
        "data-bg",
    )
}

fn parse_novel_description(tooltip: &ElementRef) -> Option<String> {
    let novel_description_wrapper = tooltip.select(&SELECTORS.novel_description).next()?;
    let novel_description_element = novel_description_wrapper.last_child()?;
    let novel_description = novel_description_element.value().as_text()?;
    Some(novel_description.trim().to_string())
}

// Enrich Helpers
fn parse_description_enrich(info_box: &ElementRef) -> Option<String> {
    let description = info_box.select(&SELECTORS.enrich_description).next()?;

    Some(element_to_markdown(&description, "\n").trim().to_string())
}

fn parse_people(info_box: &ElementRef, people_selector: &Selector, separate: &str) -> Vec<String> {
    let mut people: Vec<String> = Vec::new();
    if let Some(people_element) = info_box.select(people_selector).next() {
        let people_raw = people_element.inner_html().trim().to_string();
        people.extend(people_raw.split(separate).map(String::from));
    }
    people
}

fn parse_authors(info_box: &ElementRef, separate: &str) -> Vec<String> {
    parse_people(info_box, &SELECTORS.enrich_author, separate)
}

fn parse_artists(info_box: &ElementRef, separate: &str) -> Vec<String> {
    parse_people(info_box, &SELECTORS.enrich_artist, separate)
}

fn parse_tags(info_box: &ElementRef) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    let tag_elements = info_box
        .select(&SELECTORS.enrich_tag)
        .filter(|el| el.attr("href").is_some())
        .collect::<Vec<_>>();
    for tag_element in tag_elements {
        let tag = tag_element.inner_html().trim().to_string();
        tags.push(tag);
    }
    tags
}
