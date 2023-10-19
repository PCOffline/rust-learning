use regex::escape;
use regex::Regex;
use std::fs;
use std::io;
use std::usize;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();

    println!("Enter the path to the markdown file:");
    stdin.read_line(&mut input)?;

    let content_result = fs::read_to_string(&input.trim());
    match content_result {
        Ok(result) => parse_file_to_markdown(&result),
        Err(err) => println!("{err}"),
    }

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
        }
    }

    fn from(text: &str) -> Option<Element> {
        fn gen_regex_str_for_wrap(template: &str) -> String {
            escape(template) + r"[^" + template + "]?.*" + &escape(template)
        }

        fn gen_regex_str_for_prefix(template: &str) -> String {
            "^".to_owned() + &escape(template) + "[^" + template + "]?.+$"
        }

        let title_regex = Regex::new(&gen_regex_str_for_prefix("#")).unwrap();
        let subtitle_regex = Regex::new(&gen_regex_str_for_prefix("##")).unwrap();
        let header_regex = Regex::new(&gen_regex_str_for_prefix("###")).unwrap();
        let subheader_regex = Regex::new(&gen_regex_str_for_prefix("####")).unwrap();

        let italic_regex = Regex::new(&gen_regex_str_for_wrap("*")).unwrap();
        let bold_regex = Regex::new(&gen_regex_str_for_wrap("**")).unwrap();
        let strikethrough_regex = Regex::new(&gen_regex_str_for_wrap("~")).unwrap();
        let underline_regex = Regex::new(&gen_regex_str_for_wrap("_")).unwrap();

        let unordered_list_regex = Regex::new(r"^\s*-\s+.+$").unwrap();
        let ordered_list_regex = Regex::new(r"^\s*\d\.\s+.+$").unwrap();
        let link_regex = Regex::new(r"\[(?<text>.+)\]\((?<url>.+)\)").unwrap();
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
            return Some(Element::Title(get_trimmed_text(text, "# ")));
        } else if subtitle_regex.is_match(text) {
            return Some(Element::Subtitle(get_trimmed_text(text, "## ")));
        } else if header_regex.is_match(text) {
            return Some(Element::Header(get_trimmed_text(text, "### ")));
        } else if subheader_regex.is_match(text) {
            return Some(Element::Subheader(get_trimmed_text(text, "#### ")));
        } else if bold_regex.is_match(text) {
            return Some(Element::Bold(get_trimmed_text_from_both_sides(text, "**")));
        } else if italic_regex.is_match(text) {
            return Some(Element::Italic(get_trimmed_text_from_both_sides(text, "*")));
        } else if strikethrough_regex.is_match(text) {
            return Some(Element::Strikethrough(get_trimmed_text_from_both_sides(
                text, "~",
            )));
        } else if underline_regex.is_match(text) {
            return Some(Element::Underline(get_trimmed_text_from_both_sides(
                text, "_",
            )));
        } else if code_regex.is_match(text) {
            return Some(Element::Code(get_trimmed_text_from_both_sides(text, "`")));
        } else if code_block_regex.is_match(text) {
            return Some(Element::Code(get_trimmed_text_from_both_sides(text, "```")));
        } else if link_regex.is_match(text) {
            let captures = link_regex.captures(text).unwrap();
            return Some(Element::Link(
                &captures.name("text").unwrap().as_str(),
                &captures.name("url").unwrap().as_str(),
            ));
        } else if text.lines().all(|line| unordered_list_regex.is_match(line)) {
            return Some(Element::UnorderedList(
                text.lines()
                    .enumerate()
                    .map(|(_, line)| get_trimmed_text(&line, "- "))
                    .collect::<Vec<&str>>(),
            ));
        } else if text.lines().all(|line| ordered_list_regex.is_match(line)) {
            return Some(Element::OrderedList(
                text.lines()
                    .enumerate()
                    .map(|(_, line)| get_trimmed_text(&line, ". "))
                    .collect::<Vec<&str>>(),
            ));
        }

        None
    }
}

fn parse_file_to_markdown(content: &str) {
    Element::from(content)
        .expect("Element not identified!")
        .to_html()
}
