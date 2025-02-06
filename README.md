# m2h

`m2h` is a simple command-line tool that converts Markdown to HTML with syntax
highlighting for code blocks. It allows you to customize the styling by applying
different themes at render time.

## Install

```bash
cargo install m2h --locked
```

## Usage

### Convert Markdown to HTML

By default, `m2h` reads Markdown from standard input and outputs HTML.

```bash
$ echo "# Hello, world!" | m2h
```

```html
<h1>Hello, world!</h1>
```

### Syntax Highlighting

Code blocks in the Markdown input are annotated with CSS classes rather than
inline styles. This allows themes to be applied dynamically at render time.

#### Example

````bash
$ echo $'```rust
fn main() {
    println!("Hello, world!");
}```' | m2h
````

#### Output

```html
<pre><code class="language-rust"><span class="source rust"><span class="meta
function rust"><span class="meta function rust"><span class="storage type
function rust">fn ...
```

You can then apply a theme by supplying CSS that styles these class names.

### List Available Themes

To list all available themes, run:

```bash
m2h theme
```

### Get Theme CSS

To retrieve the CSS for a specific theme, run:

```bash
m2h theme <THEME_NAME>
```

For example:

```bash
m2h theme Monokai
```

This will output the corresponding CSS, which you can include in your HTML.

## Example: Convert and Apply a Theme

1. Convert Markdown to HTML:

   ```bash
   cat example.md | m2h > output.html
   ```

2. Retrieve the CSS for a theme:

   ```bash
   m2h theme Solarized-Dark > theme.css
   ```

3. Include `theme.css` in your webpage to style the syntax highlighting
   dynamically.

## License

MIT License
