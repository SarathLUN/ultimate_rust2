# Lesson 24 - Channels:

- Don't use `std::sync::mpsc` channels: it's an older design with limitation and won't be improved due to backwards compatibility concerns.
- Use `crossbeam` channels instead: they are faster, more efficient, and offer more features.
- Channels are one-way communication queues: threads can send and receive data through them.
- Data need to implement `Send` to be sent through channels: the compiler ensures thread safety.
- Channel can be bounded or unbounded: Bounded channels have a fixed capacity and block sender when full. Unbounded channels grow indefinitely but risk running out of memory.
- Channels can have multiple senders and/or receivers: only one receiver gets a sent value, and senders compete on first-come, first-serve basis.
- Be careful with bidirectional communication: use multiple channels and avoid cyclic dependencies to prevent deadlocks.
- Closing a channel signals the end of communication: Receiver will stop iterating when a channel is closed.
- function `drop` does nothing but releases ownership: it's useful to ensure automatic cleanup when a value goes out of the scope.

### Example code directory: l024-channels
```shell
cargo run 
```