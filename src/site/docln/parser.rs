use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector, selectable::Selectable};

use crate::{
    site::{
        content::novels::{ChapterMeta, ChapterRaw, NovelEnrich, NovelRaw},
        docln::converter::element_to_markdown,
    },
    utils::time::current_stamp,
};

use super::converter::elements_to_markdown;

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
    Html::parse_document(html)
        .select(&SELECTORS.novel_max_page)
        .next()
        .unwrap()
        .attr("href")
        .unwrap()
        .to_string()
        .split("page=")
        .nth(1)
        .and_then(|x| Some(x.trim()))
        .unwrap()
        .parse::<i64>()
        .unwrap()
}

pub fn parse_novels(html: &str) -> Vec<NovelRaw> {
    let raw = Html::parse_document(html);
    let mut result: Vec<NovelRaw> = Vec::new();

    for (_i, preview) in raw.select(&SELECTORS.novel_previews).enumerate() {
        // attr format: data-tooltip-content="#series_15056"
        let tooltip = preview
            .attr("data-tooltip-content")
            .and_then(|id| Selector::parse(id).ok())
            .and_then(|selector| raw.select(&selector).next());

        if let Some(tooltip) = tooltip {
            result.push(get_novel(&tooltip, &preview));
        }
    }

    result
}

pub fn parse_chapters_list(html: &str) -> Vec<ChapterMeta> {
    let raw = Html::parse_document(html);
    let mut result: Vec<ChapterMeta> = Vec::new();

    for chapter in raw.select(&SELECTORS.chapters) {
        let chapter = parse_chapter_meta(&chapter);
        result.push(chapter);
    }

    result
}

pub fn parse_novel_enrich(html: &str) -> NovelEnrich {
    let document = Html::parse_document(&html);
    let info_box = document.select(&SELECTORS.enrich_info_box).next().unwrap();
    let separate = "; ";
    NovelEnrich {
        description: parse_description_enrich(&info_box),
        authors: parse_authors(&info_box, separate),
        artists: parse_artists(&info_box, separate),
        tags: parse_tags(&info_box),
        updated_at: current_stamp() as i64,
    }
}

// Helpers
fn parse_attribute(element: &ElementRef, attribute: &str) -> String {
    element.attr(attribute).unwrap().trim().to_string()
}

// Content Helpers
pub fn parse_chapter_content(html: &str) -> String {
    let raw = Html::parse_document(html);
    let chapter_contents = raw.select(&SELECTORS.chapter_contents);
    elements_to_markdown(chapter_contents, "\n\n")
}

pub fn get_chapter(chapter_meta: ChapterMeta, index: i64, content: String) -> ChapterRaw {
    ChapterRaw {
        title: chapter_meta.title,
        slug: chapter_meta.slug,
        novel_id: 0,
        created_at: current_stamp() as i64,
        updated_at: current_stamp() as i64,
        content,
        chapter_number: Some(index),
    }
}

fn parse_chapter_meta(chapter: &ElementRef) -> ChapterMeta {
    ChapterMeta {
        title: parse_chapter_title(&chapter),
        slug: parse_chapter_slug(&chapter),
    }
}

fn parse_chapter_title(chapter: &ElementRef) -> String {
    parse_attribute(&chapter, "title")
}

fn parse_chapter_slug(chapter: &ElementRef) -> String {
    parse_attribute(&chapter, "href")
}

// Item Helpers
fn get_novel(tooltip: &ElementRef, preview: &ElementRef) -> NovelRaw {
    NovelRaw {
        id: parse_novel_id(tooltip),
        title: parse_novel_title(tooltip),
        slug: parse_novel_slug(preview),
        thumbnail: parse_novel_thumbnail(preview),
        description: parse_novel_description(tooltip),
        authors: Vec::new(),
        artists: Vec::new(),
        tags: Vec::new(),
        created_at: current_stamp() as i64,
        updated_at: current_stamp() as i64,
    }
}

fn parse_novel_id(tooltip: &ElementRef) -> i64 {
    parse_attribute(&tooltip, "id")
        .strip_prefix("series_")
        .unwrap()
        .parse::<i64>()
        .unwrap()
}

fn parse_novel_title(tooltip: &ElementRef) -> String {
    tooltip
        .select(&SELECTORS.novel_title)
        .next()
        .unwrap()
        .inner_html()
        .trim()
        .to_string()
}

fn parse_novel_slug(preview: &ElementRef) -> String {
    parse_attribute(
        &preview
            .parent()
            .and_then(ElementRef::wrap)
            .unwrap()
            .select(&SELECTORS.novel_slug)
            .next()
            .unwrap(),
        "href",
    )
}

fn parse_novel_thumbnail(preview: &ElementRef) -> Option<String> {
    Some(parse_attribute(
        &preview.select(&SELECTORS.novel_thumbnail).next().unwrap(),
        "data-bg",
    ))
}

fn parse_novel_description(tooltip: &ElementRef) -> Option<String> {
    Some(
        tooltip
            .select(&SELECTORS.novel_description)
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

// Enrich Helpers
fn parse_description_enrich(info_box: &ElementRef) -> Option<String> {
    let description = info_box
        .select(&SELECTORS.enrich_description)
        .next()
        .unwrap();

    Some(element_to_markdown(&description, "\n").trim().to_string())
}

fn parse_people(info_box: &ElementRef, people_selector: &Selector, separate: &str) -> Vec<String> {
    let mut people: Vec<String> = Vec::new();
    let people_element = info_box.select(people_selector).next();
    if !people_element.is_some() {
        return people;
    }

    let people_raw = people_element.unwrap().inner_html().trim().to_string();
    people.extend(people_raw.split(separate).map(String::from));
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
