# Lesson 21: Logging

1. Logging in Rust is done through a facade and not build into the standard library.
- this allows for flexibility in choosing a logging backend based on our needed.
- the `log` crate provide common interface for logging across libraries.
- this separates the act of logging from where the logs are sent (stdout, file, etc.).

2. Difference log levels control message verbosity:
- there are 5 log levels: `error`, `warn`, `info`, `debug`, and `trace`.
- each level has corresponding macros (`error!`, `warn!`, etc.) for emitting log message.
- log levels are hierarchical, `error` is lowest level message and `trace` is highest level message. And high level log will include lower log. Example: if we set log level as `error` there only error that will be logged, but if we set log level as `info` there will be logging info, warn, & error.

3. Library should use `log` crate for basic logging:
- this ensures compatibility with difference logging backends used by applications.

4. applications need an additional logging backend to route message to a destination:
- common choice including `env_logger` (simple logging to stderr), syslog, file output, or cloud services.

5. `env_logger` is a simple logging backend that reads the `RUST_LOG` environment variable for log level.
- it's a good choice for development and basic logging needs.

### Code Example Key points:
- the `Puzzle` library use the `log` crate with `info!` and `error!` macros for logging.
- [Puzzle](../puzzles/src/lib.rs)
- the `puzzle_game` application use `log` crate for application logging and added `env_logger` as dependency. 
- [puzzle_game](../puzzle_game/src/main.rs)
- setting the `RUST_LOG` environment variable allows controlling the log level output by `env_logger`.

### Additional note:
- for more advanced logging features like structures and tracing, consider the `tracing` framework.

