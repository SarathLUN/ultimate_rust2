# Publishing

### Publishing on crates.io:

- think before you publish: once you publish something, it's permanent. Don't publish private information like passwords or useless packages.
- Unique package names: currently, package names need to be unique across all users.
- authentication: you need a github account and an access token to publish.

### Preparing your package:

- Cargo.toml configuration: file in details like homepage, readme, keywords, and categories to improve discoverability.
- License: specify the license for your code.

### Publishing the crate:

- run `cargo publish`: this command checks your code, verifies the version, and uploads it to crates.io.
- automatic documentation: publish crates automatically get documentation hosted on docs.rs.

### Additional points:

- this course doesn't have an exercise on publishing due to the permanence of package names.
