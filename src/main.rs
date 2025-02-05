use std::io::{self, Read, Write};

use clap::{Parser as ClapParser, Subcommand};
use pulldown_cmark::{html, CodeBlockKind, Event, Parser as MarkdownParser, Tag, TagEnd};
use serde_json;

use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List available themes or get CSS for a specific theme
    Theme {
        /// Theme name. If not provided, lists all available themes as JSON
        #[arg(value_name = "THEME")]
        name: Option<String>,
    },
    /// Convert markdown from stdin to HTML (default command)
    Convert,
}

struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
}

impl SyntaxHighlighter {
    fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
        }
    }

    fn highlight(&self, code: &str, lang: Option<&str>) -> String {
        let syntax = match lang {
            Some(lang) => self
                .syntax_set
                .find_syntax_by_token(lang)
                .or_else(|| self.syntax_set.find_syntax_by_extension(lang)),
            None => None,
        }
        .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
            syntax,
            &self.syntax_set,
            ClassStyle::Spaced,
        );

        for line in LinesWithEndings::from(code) {
            let _ = html_generator.parse_html_for_line_which_includes_newline(line);
        }

        html_generator.finalize()
    }
}

fn handle_theme(name: Option<String>) -> Result<(), String> {
    let assets = syntect_assets::assets::HighlightingAssets::from_binary();

    match name {
        None => {
            let themes: Vec<String> = assets.themes().map(String::from).collect();
            println!(
                "{}",
                serde_json::to_string_pretty(&themes).map_err(|e| e.to_string())?
            );
            Ok(())
        }
        Some(theme_name) => {
            let theme = assets.get_theme(&theme_name);
            match syntect::html::css_for_theme_with_class_style(theme, ClassStyle::Spaced) {
                Ok(css) => {
                    println!("{}", css);
                    Ok(())
                }
                Err(e) => Err(format!("Failed to generate CSS: {}", e)),
            }
        }
    }
}

fn convert_markdown() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let highlighter = SyntaxHighlighter::new();

    let mut in_code_block = false;
    let mut current_code = String::new();
    let mut current_lang = None;

    let parser = MarkdownParser::new(&input).map(|event| match event {
        Event::Start(Tag::CodeBlock(kind)) => {
            in_code_block = true;
            current_code.clear();
            current_lang = match kind {
                CodeBlockKind::Fenced(info) => Some(info.to_string()),
                CodeBlockKind::Indented => None,
            };
            Event::Text("".into())
        }
        Event::End(TagEnd::CodeBlock) => {
            in_code_block = false;
            let highlighted = highlighter.highlight(&current_code, current_lang.as_deref());
            let mut html = String::new();
            html.push_str("<pre><code");
            if let Some(lang) = &current_lang {
                html.push_str(&format!(" class=\"language-{}\"", lang));
            }
            html.push_str(">");
            html.push_str(&highlighted);
            html.push_str("</code></pre>");
            Event::Html(html.into())
        }
        Event::Text(text) => {
            if in_code_block {
                current_code.push_str(&text);
                Event::Text("".into())
            } else {
                Event::Text(text)
            }
        }
        e => e,
    });

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    io::stdout().write_all(html_output.as_bytes())?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Convert) {
        Commands::Theme { name } => {
            if let Err(e) = handle_theme(name) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Convert => {
            if let Err(e) = convert_markdown() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
