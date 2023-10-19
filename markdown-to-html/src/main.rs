use regex::escape;
use regex::Regex;
use regex::RegexBuilder;
use std::fs;
use std::io;
use std::usize;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();

    println!("Enter the path to the markdown file:");
    stdin.read_line(&mut input)?;

    let content_result = fs::read_to_string(&input.trim())?;
    let html = parse_file_to_html(&content_result);

    write_html_file(&input, &html)?;

    Ok(())
}

enum Element<'a> {
    Title(&'a str),
    Subtitle(&'a str),
    Header(&'a str),
    Subheader(&'a str),
    Bold(&'a str),
    Italic(&'a str),
    Strikethrough(&'a str),
    Underline(&'a str),
    Code(&'a str),
    Link(&'a str, &'a str),
    Image(&'a str, &'a str),
    UnorderedList(Vec<&'a str>),
    OrderedList(Vec<&'a str>),
    Checkbox(bool, &'a str),
    PlainText(&'a str),
}

impl<'a> Element<'a> {
    fn to_html(&self) {
        match self {
            Self::Title(text) => println!("Title: {text}"),
            Self::Subtitle(text) => println!("Subtitle: {text}"),
            Self::Header(text) => println!("Header: {text}"),
            Self::Subheader(text) => println!("Subheader: {text}"),
            Self::Bold(text) => println!("Bold: {text}"),
            Self::Italic(text) => println!("Italic: {text}"),
            Self::Strikethrough(text) => println!("Strikethrough: {text}"),
            Self::Underline(text) => println!("Underline: {text}"),
            Self::Code(text) => println!("Code: {text}"),
            Self::OrderedList(vec) => println!("OrderedList: {:?}", vec),
            Self::UnorderedList(vec) => println!("UnorderedList: {:?}", vec),
            Self::Checkbox(checked, text) => {
                println!("OrderedList: {{ checked: {checked}; text: {text} }}")
            }
            Self::Link(text, url) => println!("Link: {{ text: {text}; url: {url} }}"),
            Self::Image(url, alt) => println!("Image: {{ url: {url}; alt: {alt} }}"),
            Self::PlainText(text) => println!("PlainText: {text}"),
        }
    }

    fn from(text: &str) -> Element {
        fn gen_regex_str_for_wrap(template: &str) -> String {
            escape(template) + r"[^" + template + "].*" + &escape(template)
        }

        fn gen_regex_str_for_prefix(template: &str) -> String {
            "^".to_owned() + &escape(template) + "[^" + template + "].+$"
        }

        let title_regex = Regex::new(&gen_regex_str_for_prefix("#")).unwrap();
        let subtitle_regex = Regex::new(&gen_regex_str_for_prefix("##")).unwrap();
        let header_regex = Regex::new(&gen_regex_str_for_prefix("###")).unwrap();
        let subheader_regex = Regex::new(&gen_regex_str_for_prefix("####")).unwrap();

        let italic_regex = Regex::new(&gen_regex_str_for_wrap(r"\*")).unwrap();
        let bold_regex = Regex::new(&gen_regex_str_for_wrap(r"\*\*")).unwrap();
        let strikethrough_regex = Regex::new(&gen_regex_str_for_wrap("~")).unwrap();
        let underline_regex = Regex::new(&gen_regex_str_for_wrap("_")).unwrap();

        let unordered_list_regex = Regex::new(r"^\s*-\s+.+$").unwrap();
        let ordered_list_regex = Regex::new(r"^\s*\d\.\s.+$").unwrap();
        let checkbox_regex = Regex::new(r"^\s*\[[xX]?\]\s(.+)$").unwrap();
        let link_regex = Regex::new(r"\[(?<text>.+)\]\((?<url>.+)\)").unwrap();
        let image_regex = Regex::new(r"\!\[(?<alt>.+)\]\((?<url>.+)\)").unwrap();
        let code_regex = Regex::new(&gen_regex_str_for_wrap("`")).unwrap();
        let code_block_regex = Regex::new(&gen_regex_str_for_wrap(r"\n```\n")).unwrap();

        fn get_trimmed_text<'a>(text: &'a str, pattern: &'a str) -> &'a str {
            let index_of_first_char: usize = text
                .find(pattern)
                .and_then(|index| Some(index + pattern.len()))
                .unwrap_or(0);

            &text[index_of_first_char..]
        }

        fn get_trimmed_text_from_both_sides<'a>(text: &'a str, pattern: &'a str) -> &'a str {
            let index_of_first_char: usize = text
                .find(pattern)
                .and_then(|index| Some(index + pattern.len()))
                .unwrap_or(0);

            let index_of_last_char: usize = text.rfind(pattern).unwrap_or(text.len());

            &text[index_of_first_char..index_of_last_char]
        }

        if title_regex.is_match(text) {
            return Element::Title(get_trimmed_text(text, "# "));
        } else if subtitle_regex.is_match(text) {
            return Element::Subtitle(get_trimmed_text(text, "## "));
        } else if header_regex.is_match(text) {
            return Element::Header(get_trimmed_text(text, "### "));
        } else if subheader_regex.is_match(text) {
            return Element::Subheader(get_trimmed_text(text, "#### "));
        } else if bold_regex.is_match(text) {
            return Element::Bold(get_trimmed_text_from_both_sides(text, "**"));
        } else if italic_regex.is_match(text) {
            return Element::Italic(get_trimmed_text_from_both_sides(text, "*"));
        } else if strikethrough_regex.is_match(text) {
            return Element::Strikethrough(get_trimmed_text_from_both_sides(text, "~"));
        } else if underline_regex.is_match(text) {
            return Element::Underline(get_trimmed_text_from_both_sides(text, "_"));
        } else if code_regex.is_match(text) {
            return Element::Code(get_trimmed_text_from_both_sides(text, "`"));
        } else if code_block_regex.is_match(text) {
            return Element::Code(get_trimmed_text_from_both_sides(text, "```"));
        } else if checkbox_regex.is_match(text) {
            let checked: bool = text
                .trim()
                .chars()
                .nth(1)
                .unwrap()
                .to_string()
                .to_lowercase()
                == "x";
            let checkbox_text = checkbox_regex
                .captures(text)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            return Element::Checkbox(checked, checkbox_text);
        } else if link_regex.is_match(text) {
            let captures = link_regex.captures(text).unwrap();
            return Element::Link(
                &captures.name("text").unwrap().as_str(),
                &captures.name("url").unwrap().as_str(),
            );
        } else if image_regex.is_match(text) {
            let captures = image_regex.captures(text).unwrap();
            return Element::Image(
                &captures.name("alt").unwrap().as_str(),
                &captures.name("url").unwrap().as_str(),
            );
        } else if text.lines().all(|line| unordered_list_regex.is_match(line)) {
            return Element::UnorderedList(
                text.lines()
                    .enumerate()
                    .map(|(_, line)| get_trimmed_text(&line, "- "))
                    .collect::<Vec<&str>>(),
            );
        } else if text.lines().all(|line| ordered_list_regex.is_match(line)) {
            return Element::OrderedList(
                text.lines()
                    .enumerate()
                    .map(|(_, line)| get_trimmed_text(&line, ". "))
                    .collect::<Vec<&str>>(),
            );
        }

        Element::PlainText(text)
    }
}

fn parse_file_to_html(content: &str) -> String {
    fn gen_regex(pattern: &str) -> Regex {
        RegexBuilder::new(pattern)
        .multi_line(true)
        .case_insensitive(true)
        .build()
        .unwrap()
    }

    let title_regex = gen_regex(r"^#\s(.+)$");
    let subtitle_regex = gen_regex(r"^##\s(.+)$");
    let header_regex = gen_regex(r"^###\s(.+)$");
    let subheader_regex = gen_regex(r"^####\s(.+)$");

    let italic_regex = gen_regex(r"\*(.+)\*");
    let bold_regex = gen_regex(r"\*\*(.+)\*\*");
    let strikethrough_regex = gen_regex("~(.+)~");
    let underline_regex = gen_regex("_(.+)_");

    let unordered_list_regex = gen_regex(r"^\s*-\s+(.+)$");
    let ordered_list_regex = gen_regex(r"^\s*\d\.\s(.+)$");
    let checkbox_regex = gen_regex(r"^\s*\[x?\]\s(.+)$");
    let link_regex = gen_regex(r"\[(?<text>.+)\]\((?<url>.+)\)");
    let image_regex = gen_regex(r"\!\[(?<alt>.+)\]\((?<url>.+)\)");
    let code_regex = gen_regex("`(.+)`");
    let code_block_regex = gen_regex(r"^\s*```\w*\n((.+\n*)+)\n```$");

    let content = title_regex.replace_all(content, "<h1>$1</h1>").to_string();
    let content = subtitle_regex.replace_all(&content, "<h2>$1</h2>").to_string();
    let content = subtitle_regex.replace_all(&content, "<h2>$1</h2>").to_string();
    let content = header_regex.replace_all(&content, "<h3>$1</h3>").to_string();
    let content = subheader_regex.replace_all(&content, "<h4>$1</h4>").to_string();
    let content = bold_regex.replace_all(&content, "<b>$1</b>").to_string();
    let content = italic_regex.replace_all(&content, "<i>$1</i>").to_string();
    let content = strikethrough_regex.replace_all(&content, "<s>$1</s>").to_string();
    let content = underline_regex.replace_all(&content, "<u>$1</u>").to_string();
    let content = code_block_regex.replace_all(&content, "<pre><code>$1</code><pre>").to_string();
    let content = code_regex.replace_all(&content, "<code>$1</code>").to_string();
    let content = image_regex.replace_all(&content, "<img src=\"$url\" alt=\"$alt\">").to_string();
    let content = link_regex.replace_all(&content, "<a href=\"$url\">$text</a>").to_string();
    // let content = subtitle_regex.replace_all(&content, "<h2>$1</h2>").to_string();
    // let content = subtitle_regex.replace_all(&content, "<h2>$1</h2>").to_string();
    // let content = subtitle_regex.replace_all(&content, "<h2>$1</h2>").to_string();
    content.to_string()
}

fn write_html_file<'a>(file_name: &str, html: &str) -> io::Result<()> {
    let html_tree = "<html><head><title>Markdown to HTML</title></head><body>".to_owned()
        + html
        + "</body></html>";
    fs::write(file_name.replace(".md", ".html"), html_tree)
}
