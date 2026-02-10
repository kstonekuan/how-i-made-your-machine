use std::fmt::Write as _;
use std::{env, io};

use mdbook_preprocessor::book::{Book, BookItem};
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::{MDBOOK_VERSION, Preprocessor, PreprocessorContext, parse_input};
use regex::Regex;
use semver::{Version, VersionReq};

struct LanguageTabsPreprocessor;

struct ParsedTabItem {
    tab_label: String,
    tab_value: String,
    code_language: String,
    code_content: String,
    is_default_tab: bool,
}

impl Preprocessor for LanguageTabsPreprocessor {
    fn name(&self) -> &'static str {
        "language-tabs"
    }

    fn run(&self, _context: &PreprocessorContext, book: Book) -> Result<Book> {
        let mut transformed_book = book;
        transform_book_items(&mut transformed_book.items);
        Ok(transformed_book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool> {
        Ok(renderer == "html")
    }
}

fn transform_book_items(book_items: &mut [BookItem]) {
    for book_item in book_items {
        match book_item {
            BookItem::Chapter(chapter) => {
                chapter.content = transform_docusaurus_tabs_blocks(&chapter.content);
                transform_book_items(&mut chapter.sub_items);
            }
            BookItem::Separator | BookItem::PartTitle(_) => {}
        }
    }
}

fn transform_docusaurus_tabs_blocks(markdown_content: &str) -> String {
    let mut transformed_markdown = String::with_capacity(markdown_content.len());
    let mut search_start_index = 0usize;
    let tabs_open_marker = "<Tabs";
    let tabs_close_marker = "</Tabs>";

    while let Some(relative_tabs_start_index) =
        markdown_content[search_start_index..].find(tabs_open_marker)
    {
        let tabs_start_index = search_start_index + relative_tabs_start_index;
        transformed_markdown.push_str(&markdown_content[search_start_index..tabs_start_index]);

        let Some(open_tag_end_relative_index) = markdown_content[tabs_start_index..].find('>')
        else {
            transformed_markdown.push_str(&markdown_content[tabs_start_index..]);
            return transformed_markdown;
        };
        let tabs_open_tag_end_index = tabs_start_index + open_tag_end_relative_index;

        let Some(close_tag_start_relative_index) =
            markdown_content[tabs_open_tag_end_index + 1..].find(tabs_close_marker)
        else {
            transformed_markdown.push_str(&markdown_content[tabs_start_index..]);
            return transformed_markdown;
        };
        let tabs_close_tag_start_index =
            tabs_open_tag_end_index + 1 + close_tag_start_relative_index;

        let tabs_open_tag = &markdown_content[tabs_start_index..=tabs_open_tag_end_index];
        let tabs_inner_content =
            &markdown_content[tabs_open_tag_end_index + 1..tabs_close_tag_start_index];

        if let Some(rendered_tabs_html) = render_tabs_group_html(tabs_open_tag, tabs_inner_content)
        {
            transformed_markdown.push('\n');
            transformed_markdown.push_str(rendered_tabs_html.trim());
            transformed_markdown.push_str("\n\n");
        } else {
            transformed_markdown.push_str(
                &markdown_content
                    [tabs_start_index..tabs_close_tag_start_index + tabs_close_marker.len()],
            );
        }

        search_start_index = tabs_close_tag_start_index + tabs_close_marker.len();
    }

    transformed_markdown.push_str(&markdown_content[search_start_index..]);
    transformed_markdown
}

fn render_tabs_group_html(tabs_open_tag: &str, tabs_inner_content: &str) -> Option<String> {
    let group_identifier_regex = Regex::new(r#"groupId\s*=\s*"([^"]+)""#).ok()?;
    let raw_group_identifier = group_identifier_regex
        .captures(tabs_open_tag)
        .and_then(|captures| captures.get(1).map(|value| value.as_str().to_string()))
        .unwrap_or_else(|| "language-tabs-group".to_string());

    let tab_item_regex =
        Regex::new(r"(?s)<TabItem(?P<attributes>[^>]*)>(?P<content>.*?)</TabItem>").ok()?;
    let mut parsed_tab_items: Vec<ParsedTabItem> = Vec::new();

    for tab_item_capture in tab_item_regex.captures_iter(tabs_inner_content) {
        let tab_item_attributes = tab_item_capture.name("attributes")?.as_str();
        let tab_item_content = tab_item_capture.name("content")?.as_str();

        let tab_label = parse_attribute_value(tab_item_attributes, "label")
            .or_else(|| parse_attribute_value(tab_item_attributes, "value"))
            .unwrap_or_else(|| format!("Tab {}", parsed_tab_items.len() + 1));

        let tab_value = parse_attribute_value(tab_item_attributes, "value")
            .unwrap_or_else(|| sanitize_identifier(&tab_label));

        let is_default_tab = tab_item_attributes.contains("default");
        let (code_language, code_content) = extract_first_fenced_code_block(tab_item_content)?;

        parsed_tab_items.push(ParsedTabItem {
            tab_label,
            tab_value,
            code_language,
            code_content,
            is_default_tab,
        });
    }

    if parsed_tab_items.is_empty() {
        return None;
    }

    let active_tab_index = parsed_tab_items
        .iter()
        .position(|parsed_tab_item| parsed_tab_item.is_default_tab)
        .unwrap_or(0);

    let sanitized_group_identifier = sanitize_identifier(&raw_group_identifier);
    let mut rendered_tabs_html = String::new();
    write_tabs_group_start(&mut rendered_tabs_html, &sanitized_group_identifier);
    write_tabs_group_buttons(
        &mut rendered_tabs_html,
        &parsed_tab_items,
        active_tab_index,
        &sanitized_group_identifier,
    );
    write_tabs_group_panels(
        &mut rendered_tabs_html,
        &parsed_tab_items,
        active_tab_index,
        &sanitized_group_identifier,
    );
    write_tabs_group_end(&mut rendered_tabs_html);

    Some(rendered_tabs_html)
}

fn write_tabs_group_start(rendered_tabs_html: &mut String, sanitized_group_identifier: &str) {
    let _ = writeln!(
        rendered_tabs_html,
        r#"<div class="language-tabs" data-language-tabs-group="{}">"#,
        escape_html_attribute_value(sanitized_group_identifier),
    );
    let _ = writeln!(
        rendered_tabs_html,
        r#"<div class="language-tabs-list" role="tablist" aria-label="Programming language tabs">"#,
    );
}

fn write_tabs_group_buttons(
    rendered_tabs_html: &mut String,
    parsed_tab_items: &[ParsedTabItem],
    active_tab_index: usize,
    sanitized_group_identifier: &str,
) {
    for (tab_index, parsed_tab_item) in parsed_tab_items.iter().enumerate() {
        let is_active_tab = tab_index == active_tab_index;
        let tab_button_identifier = format!(
            "language-tabs-{}-button-{}",
            sanitized_group_identifier,
            sanitize_identifier(&parsed_tab_item.tab_value),
        );
        let tab_panel_identifier = format!(
            "language-tabs-{}-panel-{}",
            sanitized_group_identifier,
            sanitize_identifier(&parsed_tab_item.tab_value),
        );

        let _ = writeln!(
            rendered_tabs_html,
            r#"<button class="language-tabs-trigger{}" type="button" role="tab" id="{}" aria-controls="{}" aria-selected="{}" data-language-tabs-value="{}">{}</button>"#,
            if is_active_tab { " is-active" } else { "" },
            escape_html_attribute_value(&tab_button_identifier),
            escape_html_attribute_value(&tab_panel_identifier),
            if is_active_tab { "true" } else { "false" },
            escape_html_attribute_value(&parsed_tab_item.tab_value),
            escape_html_text_content(&parsed_tab_item.tab_label),
        );
    }

    rendered_tabs_html.push_str("</div>\n");
    rendered_tabs_html.push_str(r#"<div class="language-tabs-panels">"#);
    rendered_tabs_html.push('\n');
}

fn write_tabs_group_panels(
    rendered_tabs_html: &mut String,
    parsed_tab_items: &[ParsedTabItem],
    active_tab_index: usize,
    sanitized_group_identifier: &str,
) {
    for (tab_index, parsed_tab_item) in parsed_tab_items.iter().enumerate() {
        let is_active_tab = tab_index == active_tab_index;
        let tab_button_identifier = format!(
            "language-tabs-{}-button-{}",
            sanitized_group_identifier,
            sanitize_identifier(&parsed_tab_item.tab_value),
        );
        let tab_panel_identifier = format!(
            "language-tabs-{}-panel-{}",
            sanitized_group_identifier,
            sanitize_identifier(&parsed_tab_item.tab_value),
        );
        let encoded_code_content =
            escape_html_text_content(&parsed_tab_item.code_content).replace('\n', "&#10;");

        let _ = writeln!(
            rendered_tabs_html,
            r#"<section class="language-tabs-panel{}" role="tabpanel" id="{}" aria-labelledby="{}" data-language-tabs-value="{}">"#,
            if is_active_tab { " is-active" } else { "" },
            escape_html_attribute_value(&tab_panel_identifier),
            escape_html_attribute_value(&tab_button_identifier),
            escape_html_attribute_value(&parsed_tab_item.tab_value),
        );
        let _ = writeln!(
            rendered_tabs_html,
            r#"<pre><code class="language-{}">{}</code></pre>"#,
            escape_html_attribute_value(&parsed_tab_item.code_language),
            encoded_code_content,
        );
        rendered_tabs_html.push_str("</section>\n");
    }
}

fn write_tabs_group_end(rendered_tabs_html: &mut String) {
    rendered_tabs_html.push_str("</div>\n");
    rendered_tabs_html.push_str("</div>\n");
}

fn extract_first_fenced_code_block(tab_item_content: &str) -> Option<(String, String)> {
    let fenced_code_block_regex =
        Regex::new(r"(?s)```(?P<language>[^\r\n`]*)\r?\n(?P<code>.*?)\r?\n```").ok()?;
    let capture = fenced_code_block_regex.captures(tab_item_content)?;

    let raw_code_language = capture.name("language").map_or_else(
        || "text".to_string(),
        |value| value.as_str().trim().to_string(),
    );
    let normalized_code_language = if raw_code_language.is_empty() {
        "text".to_string()
    } else {
        raw_code_language
    };

    let code_content = capture.name("code")?.as_str().to_string();
    Some((normalized_code_language, code_content))
}

fn parse_attribute_value(attribute_source: &str, attribute_name: &str) -> Option<String> {
    let attribute_regex_pattern = format!(r#"{attribute_name}\s*=\s*"([^"]+)""#);
    let attribute_regex = Regex::new(&attribute_regex_pattern).ok()?;
    let capture = attribute_regex.captures(attribute_source)?;
    Some(capture.get(1)?.as_str().to_string())
}

fn sanitize_identifier(raw_identifier: &str) -> String {
    let mut sanitized_identifier = String::new();
    let mut previous_was_separator = false;

    for character in raw_identifier.chars() {
        if character.is_ascii_alphanumeric() {
            sanitized_identifier.push(character.to_ascii_lowercase());
            previous_was_separator = false;
        } else if !previous_was_separator {
            sanitized_identifier.push('-');
            previous_was_separator = true;
        }
    }

    let trimmed_identifier = sanitized_identifier.trim_matches('-');
    if trimmed_identifier.is_empty() {
        "language-tab".to_string()
    } else {
        trimmed_identifier.to_string()
    }
}

fn escape_html_text_content(text_content: &str) -> String {
    text_content
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_html_attribute_value(attribute_value: &str) -> String {
    escape_html_text_content(attribute_value)
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn main() -> Result<()> {
    let language_tabs_preprocessor = LanguageTabsPreprocessor;
    let argument_list: Vec<String> = env::args().collect();

    if let Some(command_name) = argument_list.get(1)
        && command_name == "supports"
    {
        let renderer_name = argument_list.get(2).map_or("", String::as_str);
        let supports_renderer = language_tabs_preprocessor
            .supports_renderer(renderer_name)
            .unwrap_or(false);
        std::process::exit(i32::from(!supports_renderer));
    }

    let (preprocessor_context, input_book) = parse_input(io::stdin())?;
    let version_requirement = VersionReq::parse(&format!(">= {MDBOOK_VERSION}"))?;
    let current_mdbook_version = Version::parse(&preprocessor_context.mdbook_version)?;

    if !version_requirement.matches(&current_mdbook_version) {
        eprintln!(
            "Warning: The language tabs preprocessor was built against mdBook {}, but is running with {}",
            MDBOOK_VERSION, preprocessor_context.mdbook_version,
        );
    }

    let processed_book = language_tabs_preprocessor.run(&preprocessor_context, input_book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;
    Ok(())
}
