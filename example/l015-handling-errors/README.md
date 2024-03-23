# Lesson 15: Handling Errors

### Key takeaways:

1. Error Handling with `Result` enum:
    - Rust use the `Result` enum to represent success (`Ok(value)`) or failure (`Err(error)`) scenarios.
    - This enforces explicit handling of potential error at compile time.
2. Unrecoverable with `panic!` macro:
    - use `panic!` macro for unrecoverable situation where the program cannot proceed (e.g. internal bugs.)
    - this should be the last resort and avoided in libraries.
3. Handling error with `if let `:
    - use `if let` check for specific variants in the `Result` enum when you care more about the error than the success value.
4. Default value with `match` expression:
    - use `match` expression to handle both success and error cases in the result enum, potentially providing default value for error.
5. Propagating Errors:
    - if a function cannot handle an error itself, it should return error (`Err(value)`) for the caller to handle.
6. Error chaining with `?` operator (question mark):
    - The `?` operator simplifies error handling for function returning `Result`.
    - If the `Result` is `Err(error)`, the error is propagated up the caller stack.
7. Handling multiple Error types with traits:
    - Use traits like `Error` to allow function to handle difference error types uniformly.
    - Libraries like `anyhow` provide conveniences for working with various error types.
    - code example: https://github.com/SarathLUN/ultimate_rust2/tree/main/example/puzzle_game
