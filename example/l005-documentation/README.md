# Lesson 5: Documentation

### generating documentation:
- use `cargo doc` to generate documentation website.
- use `--no-deps` to exclude dependencies from documentation.
- use `--open` to automaticlly open the documentation in your browser.

### documenting public items:
- public items like constants are automatically included in the documentation.
- private items can be documented, but require a special flag for `cargo doc`.

### basic documentation structure:
- documentation comments begin with `//` followed by a space and your documentation text.
- markdown syntax is used for formatting within documentation comments.
- headers (`#`) are encouraged to improve readability.
- start with a descriptive first paragraph for the index page.

### link within documentation:
- use `text: path` for intra-doc links to other documented items within the project.
- backticks around the text format it as code.
- square brackets without brackticks use normal text format.
- absolute paths can be used for items not in scope.

### type of documentation comments:
- outer comments: used for items outside of the code block they document (e.g., struct, modules, constants).
- inner comments: used for items within the code block they document (e.g., libraries, modules within `lib.rs`).

### documenting structs and finctions:
- document the entire struct at the top, and each field separately.
- document individual functions and methods above their definition.
- implementation blocks are generally not documented separately.

### additional trips:
- explore the full capabilities of markdown for formatting your documentation.
- utilize the generated documentation website's features like search and comtomization.
