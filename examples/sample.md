## m2h (Markdown to HTML)

This is a simple CLI tool that converts markdown to HTML.

```bash
$ cat sample.md | m2h
```

```html
<h2>m2h (Markdown to HTML)</h2>
<p>This is a simple CLI tool that converts markdown to HTML.</p>
...
```

```
This block doesn't have a type
```

```rust
use std::io::{self, Read, Write};

use clap::{Parser as ClapParser, Subcommand};
use pulldown_cmark::{html, CodeBlockKind, Event, Parser as MarkdownParser, Tag, TagEnd};

use syntect::highlighting::ThemeSet;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

// ... rest of the code remains the same ...
```

The changes are:

1. Renamed clap's `Parser` to `ClapParser` using
   `use clap::{Parser as ClapParser, Subcommand};`
2. Renamed pulldown-cmark's `Parser` to `MarkdownParser` using
   `use pulldown_cmark::{html, CodeBlockKind, Event, Parser as MarkdownParser, Tag, TagEnd};`
3. Updated the derive macro to use `#[derive(ClapParser)]`

Now the code should compile without any naming conflicts.
