# Lesson 23 - Multithreading:

- Rust provides a portable API for working with threads.
- Threads are a lightweight way to parallelize tasks within a program compared to separate processes.
- They are ideal for utilizing multiple CPU cores to portentially speed up computations.
- Threads share memory within the same process, enabling communication and data exachange.
- However, creating and managing threads introduces overhead due to context switching between threads.
- Async/await is a more efficient approach for waiting on operation like I/O bound tasks.
- The `thread::spawn` function creates a new thread and the `join` method waits for the thread to finish.
- Only data implementing the `Send` trait can be transferred between threads.

### Example directory: l023-multithreading

```shell
RUST_LOG=info cargo run
```
