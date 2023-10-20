use regex::Captures;
use regex::Regex;
use regex::RegexBuilder;

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
    CodeBlock(&'a str),
    Link(&'a str, &'a str),
    Image(&'a str, &'a str),
    UnorderedList(Vec<&'a str>),
    OrderedList(Vec<&'a str>),
    Checkbox(bool, &'a str),
}

impl<'a> Element<'a> {
    fn to_html(&self) -> String {
        match self {
            Self::Title(text) => format!("<h1>{text}</h1>"),
            Self::Subtitle(text) => format!("<h2>{text}</h2>"),
            Self::Header(text) => format!("<h3>{text}</h3>"),
            Self::Subheader(text) => format!("<h4>{text}</h4>"),
            Self::Bold(text) => format!("<b>{text}</b>"),
            Self::Italic(text) => format!("<i>{text}</i>"),
            Self::Strikethrough(text) => format!("<s>{text}</s>"),
            Self::Underline(text) => format!("<u>{text}</u>"),
            Self::CodeBlock(text) => format!("<pre><code>{text}</code></pre>"),
            Self::Code(text) => format!("<code>{text}</code>"),
            Self::OrderedList(vec) => format!(
                "<ol>{}</ol>",
                vec.iter()
                    .map(|item| format!("<li>{item}</li>"))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            Self::UnorderedList(vec) => format!(
                "<ul>{}</ul>",
                vec.iter()
                    .map(|item| format!("<li>{item}</li>"))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            Self::Checkbox(checked, text) => {
                format!("<input type=\"checkbox\" checked=\"{checked}\"><label>{text}</label>")
            }
            Self::Link(text, url) => format!("<a href=\"{url}\">{text}</a>"),
            Self::Image(alt, url) => format!("<img src=\"{url}\" alt=\"{alt}\">"),
        }
    }
}

pub fn markdown_to_html(text: &str) -> String {
    fn gen_regex(pattern: &str) -> Regex {
        RegexBuilder::new(pattern)
            .multi_line(true)
            .case_insensitive(true)
            .build()
            .unwrap()
    }

    fn gen_prefix_regex(pattern: &str) -> Regex {
        gen_regex(&(String::from(r"^\s*") + pattern + r"(?<text>.+)$"))
    }

    fn gen_wrap_regex(pattern: &str) -> Regex {
        gen_regex(&(pattern.to_owned() + r"(?<text>.+)" + pattern))
    }

    fn list_str_to_vec(list_str: &str) -> Vec<&str> {
        list_str.split("\n").filter(|str| str.trim() != "").map(|str| &str.trim()[str.find(' ').unwrap() + 1..]).collect::<Vec<&str>>()
    }

    let title_regex = gen_prefix_regex("#");
    let subtitle_regex = gen_prefix_regex("##");
    let header_regex = gen_prefix_regex("###");
    let subheader_regex = gen_prefix_regex("####");

    let italic_regex = gen_wrap_regex(r"\*");
    let bold_regex = gen_wrap_regex(r"\*\*");
    let strikethrough_regex = gen_wrap_regex("~");
    let underline_regex = gen_wrap_regex("_");

    let unordered_list_regex = gen_regex(r"(^\s*-\s.+\n*)+");
    let ordered_list_regex = gen_regex(r"(^\s*\d+\.\s.+\n*)+");
    let checkbox_regex = gen_prefix_regex(r"\[(?<checkmark>x?)\]\s");
    let link_regex = gen_regex(r"\[(?<text>.+)\]\((?<url>.+)\)");
    let image_regex = gen_regex(r"\!\[(?<alt>.*)\]\((?<url>.+)\)");
    let code_regex = gen_wrap_regex("`");
    let code_block_regex = gen_regex(r"^\s*```\w*\n(?<text>(.+\n*)+)\n```\s*$");

    let text = title_regex.replace_all(text, |captures: &Captures| Element::Title(captures.name("text").unwrap().as_str()).to_html());
    let text = subtitle_regex.replace_all(&text, |captures: &Captures| Element::Subtitle(captures.name("text").unwrap().as_str()).to_html());
    let text = header_regex.replace_all(&text, |captures: &Captures| Element::Header(captures.name("text").unwrap().as_str()).to_html());
    let text = subheader_regex.replace_all(&text, |captures: &Captures| Element::Subheader(captures.name("text").unwrap().as_str()).to_html());
    let text = bold_regex.replace_all(&text, |captures: &Captures| Element::Bold(captures.name("text").unwrap().as_str()).to_html());
    let text = italic_regex.replace_all(&text, |captures: &Captures| Element::Italic(captures.name("text").unwrap().as_str()).to_html());
    let text = strikethrough_regex.replace_all(&text, |captures: &Captures| Element::Strikethrough(captures.name("text").unwrap().as_str()).to_html());
    let text = underline_regex.replace_all(&text, |captures: &Captures| Element::Underline(captures.name("text").unwrap().as_str()).to_html());
    let text = code_block_regex.replace_all(&text, |captures: &Captures| Element::CodeBlock(captures.name("text").unwrap().as_str()).to_html());
    let text = code_regex.replace_all(&text, |captures: &Captures| Element::Code(captures.name("text").unwrap().as_str()).to_html());
    let text = image_regex.replace_all(&text, |captures: &Captures| Element::Image(captures.name("alt").unwrap().as_str(), captures.name("url").unwrap().as_str()).to_html());
    let text = link_regex.replace_all(&text, |captures: &Captures| Element::Link(captures.name("text").unwrap().as_str(), captures.name("url").unwrap().as_str()).to_html());
    let text = checkbox_regex.replace_all(&text, |captures: &Captures| Element::Checkbox(captures.name("checkmark").unwrap().as_str() != "", captures.name("text").unwrap().as_str()).to_html());
    let text = unordered_list_regex.replace_all(&text, |captures: &Captures| Element::UnorderedList(list_str_to_vec(captures.get(0).unwrap().as_str())).to_html());
    let text = ordered_list_regex.replace_all(&text, |captures: &Captures| Element::OrderedList(list_str_to_vec(captures.get(0).unwrap().as_str())).to_html());

    text.to_string()
}