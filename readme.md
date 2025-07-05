# Simple Markdown to HTML Compiler

A *tiny* Markdown compiler written in `Rust`.  
It turns `.md` into valid HTML.

---

## Features

Here are some **features**:

- Parse headings (`#`, `##`, `###`, etc.)
- Render *bold* and _italic_ text.
- Handle inline code like `let x = 5;`.
- Convert [links](https://example.com) and ![images](https://via.placeholder.com/150).
- Line & column tracking for better error reporting

---

## ⚡️ Why?

 - To learn how real compilers work, end-to-end.
 - To understand tokenizing, parsing, and rendering — by building it, not just reading about it.
 - To have a dead-simple, blazing-fast tool for converting .md to .html.
 - Maybe build a custom templating language for markdown.

---

## 🚧 Planned

Lists (-, *, 1.)
Blockquotes (>)
Nested inlines (**bold *italic***)
Basic error handling for unmatched markers
Minimal CSS output (optional)

---

## 📦 Usage

- Write your markdown in the `k.md` file.
- run `cargo run`.
- open the output.html file in your browser. 

```bash
cargo run && open output.html
```

---

## 🧩 How It Works

- Lexer — breaks the input .md text into tokens (headings, text, markers, links).
- Parser — turns the token stream into an AST (syntax tree).
- Renderer — walks the AST and writes valid HTML.
- CLI — glues it all together.

