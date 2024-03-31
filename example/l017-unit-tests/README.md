# Lesson 17: Unit Tests

### Writing Unit Tests:

- Unit tests are written to test small units of code in isolation.
- Tests reside in their own submodule named `test` at the bottom of the same file as the code being tested.
- the `cfg` attribute with `test` argument ensure the test module is only complied when running tests.
- Tests can access code from parent module using `super::*`.
- a test function is marked with the `test` attribute and has no parameters, return nothing (typically `()`), or a `Result`.
- common assertions macro include `assert_ed!()` for equality, `assert_ne!()` for non-equality, and custom logic using `assert!()`.
- the `#[should_panic]` attribute indicates a test expects a panic and should pass if it panic.

### Running Tests:

- the `cargo test` command runs unit tests.

### Test output:

- cargo test output show compilation time and the name of the complied binary for running tests.
- each test get a line indicating the test name and pass/fail status.
- a summary for each crate (library or binary) includes the overall result and counts of passed/failed tests.
- Doc-test, which test code snippets in documentation, are included in a separate section.
- Doc-tests are written as code examples within the documentation using fenced code blocks.

### Additional Notes:

- there are two definitions of "crate" in Rust:
  - Package: contains zero or one library and any number of binaries.
  - Library or Binary: each is considered a separate crate for testing purposes.
- unit tests can optionally return a `Result` type to leverage the `?` operator for concise error handling.
- by default, Rust tests fail fast, stopping after encountering the first test failure.
- it's recommended to run specific failing tests during developing using `cargo test` with the test path.
