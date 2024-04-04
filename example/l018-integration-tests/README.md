# Integration Tests in Rust: Key Takeaways

This lesson from the Ultimate Rust crash course 2 covered how to write integration tests for Rust projects. Here are the key points:

1. Separate directory: integration tests reside in a dedicated `tests` directory at the project root, alongside the `src` directory containing your library code.
2. Test file naming: any Rust within the `tests` directory considered for tests. The lesson use `anything.rs` as an example.
3. Mocking vs Integration: integration tests focus on how difference parts of your library works together, mimicking real world usage. This differ from unit tests that isolate and test individual function.
4. Test structure: similar to unit tests, integration test use `#[test]` attribute and follow the same function signature for test functions. You'll also use familiar assertion macros like `assert_eq!()`.
5. Limited scope: in this example the library has only single function `snuggle()` so the integration test might resemble the unit test. This is there aren't many components to integration yet. 
6. Testing binary: the recommended approach is to minimize logic in your binary and delegate functionality to the library. Ideally, the binary becomes so simple that testing becomes unnecessary. However, if required, tools like `std::process::Command` can be used to execute the binary as a subprocess for testing purposes.

