use std::fs;
use std::io;
mod parse;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();

    println!("Enter the path to the markdown file:");
    stdin.read_line(&mut input)?;

    let content_result = fs::read_to_string(&input.trim())?;
    let html = parse::markdown_to_html(&content_result);
    write_html_file(&input, &html)?;

    Ok(())
}

fn write_html_file<'a>(file_name: &str, html: &str) -> io::Result<()> {
    let html_tree = "<html><head><title>Markdown to HTML</title></head><body>".to_owned()
        + html
        + "</body></html>";
    fs::write(file_name.replace(".md", ".html"), html_tree)
}
