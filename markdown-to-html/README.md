# Markdown to HTML

A project that takes a markdown file and creates an HTML file from it.
This project is for learning purposes and sums up most of Rust basics.

## How It Works

There are four main stages:
1. **Input from the user**: it's actually pretty simple, we use the terminal to ask the user for the path to the file, validate the file exists and send it down the line.
2. **Find patterns with RegEx**: basically, we write a lot of regular expressions that know to identify markdown syntax (e.g. \*bold text\*)
3. **Transform text to elements**: in short, we have a cool enum that represents each element (e.g. link, image, bold text, etc.) to which we send our text split to relevant parts (for example, in links, there's the URL part and the text part). Now that we have all the parts, we can easily create an HTML element with a simple template string.
4. **Create a combined HTML file**: we create an HTML file, build a basic DOM tree, and write to it all the elements we've gathered.

## Challenges to Tackle
1. The regular expressions are a bit tricky to get right from the first time, especially if you're not experienced with writing them.
2. You need to know where a part begins and where it ends. If you send the entire file at once to parse, it will either fail or catch only one markdown element, so you have to think of a smart way to split the file to its sections.
3. There are a bunch of edge-cases and fine-tuning, for example combinations of bold, italic, strikethrough, underline, lists, etc.
4. You want to make sure that your code is safe, so that even if something fails, you can just treat is as plaintext and keep parsing the rest.
