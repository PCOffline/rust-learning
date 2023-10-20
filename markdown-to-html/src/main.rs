use regex::Regex;
use regex::RegexBuilder;
use std::env::args;
use std::fs;
use std::io;
mod over_engineered;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();

    // TODO: Use over-engineered version if true
    let use_over_engineered_version = args()
        .nth(1)
        .and_then(|arg| Some(arg == "--over_engineered" || arg == "-oe"))
        .unwrap_or(false);

    println!("Enter the path to the markdown file:");
    stdin.read_line(&mut input)?;

    let content_result = fs::read_to_string(&input.trim())?;
    let mut html;

    if (use_over_engineered_version) {
        html = over_engineered::parse_markdown_to_html(&content_result);
    } else {
        html = parse_markdown_to_html(&content_result);
    }

    write_html_file(&input, &html)?;

    Ok(())
}

fn parse_markdown_to_html(content: &str) -> String {
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
    let content = subtitle_regex
        .replace_all(&content, "<h2>$1</h2>")
        .to_string();
    let content = subtitle_regex
        .replace_all(&content, "<h2>$1</h2>")
        .to_string();
    let content = header_regex
        .replace_all(&content, "<h3>$1</h3>")
        .to_string();
    let content = subheader_regex
        .replace_all(&content, "<h4>$1</h4>")
        .to_string();
    let content = bold_regex.replace_all(&content, "<b>$1</b>").to_string();
    let content = italic_regex.replace_all(&content, "<i>$1</i>").to_string();
    let content = strikethrough_regex
        .replace_all(&content, "<s>$1</s>")
        .to_string();
    let content = underline_regex
        .replace_all(&content, "<u>$1</u>")
        .to_string();
    let content = code_block_regex
        .replace_all(&content, "<pre><code>$1</code><pre>")
        .to_string();
    let content = code_regex
        .replace_all(&content, "<code>$1</code>")
        .to_string();
    let content = image_regex
        .replace_all(&content, "<img src=\"$url\" alt=\"$alt\">")
        .to_string();
    let content = link_regex
        .replace_all(&content, "<a href=\"$url\">$text</a>")
        .to_string();
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
