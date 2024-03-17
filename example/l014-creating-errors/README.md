# Learn about Errors:

1. Errors as Enums: define error type as Enums for better organization and handling.
2. Grouping Errors: grouping related errors in same Enums for efficient handling.
3. Isolating Library Errors: Don't expose errors from external library directly. Wrap them in your own error type.
4. Non-Exhaustive Enums: use `#[non-exhaustive]` attribute on error Enums to allow for future error variant additions without break user code.
5. Implementing Error Traits:
    - manual approach (less concise): implement `Debug`, `Display`, and `Error` traits manually.
    - `thiserror` crate (recommended): use the `thiserror` to automatically implement `Display` and `Error` traits.

