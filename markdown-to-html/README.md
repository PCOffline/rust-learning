# Markdown to HTML

A project that takes a markdown file and creates an HTML file from it.
This project is for learning purposes and sums up most of Rust basics.

## How It Works

There are four stages:
1. **Input from the user**: use the terminal to ask the user for the path to the file, validate that the file exists and send it down the line.
2. **Find patterns with RegEx**: regular expressions that know to identify markdown syntax (e.g. \*bold text\*)
3. **Transform text to elements**: using an enum that represents all the different elements (title, image, link, etc.) we can define what parameters each element needs, and create a method that transforms that element into an HTML string
4. **Create a combined HTML file**: create an HTML file, build a basic DOM tree, and write to it all the elements we've gathered.

## Challenges to Tackle
1. The regular expressions are a bit tricky to get right from the first time, especially if you're not experienced with writing them.
2. There are a bunch of edge-cases and fine-tuning, for example combinations of bold, italic, strikethrough, underline, lists, etc.
