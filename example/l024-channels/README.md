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
RUST_LOG=info cargo run 
```

```shell
[2024-04-16T05:13:39Z INFO  l024_channels] ORDER: polish dog
[2024-04-16T05:13:39Z INFO  l024_channels] ORDER: caesar salad
[2024-04-16T05:13:39Z INFO  l024_channels] zack receives an order for polish dog
[2024-04-16T05:13:39Z INFO  l024_channels] ORDER: onion soup
[2024-04-16T05:13:39Z INFO  l024_channels] alice receives an order for caesar salad
[2024-04-16T05:13:39Z INFO  l024_channels] ORDER: reuben sandwich
[2024-04-16T05:13:40Z INFO  l024_channels] zack sends a HotDog
[2024-04-16T05:13:40Z INFO  l024_channels] zack receives an order for onion soup
[2024-04-16T05:13:40Z INFO  l024_channels] Order Up! -> HotDog
[2024-04-16T05:13:40Z INFO  l024_channels] alice sends a Salad
[2024-04-16T05:13:40Z INFO  l024_channels] alice receives an order for reuben sandwich
[2024-04-16T05:13:40Z INFO  l024_channels] Order Up! -> Salad
[2024-04-16T05:13:41Z INFO  l024_channels] zack sends a Soup
[2024-04-16T05:13:41Z INFO  l024_channels] Order Up! -> Soup
[2024-04-16T05:13:42Z INFO  l024_channels] alice sends a Sandwich
[2024-04-16T05:13:42Z INFO  l024_channels] Order Up! -> Sandwich
[2024-04-16T05:13:42Z INFO  l024_channels] Shop closed!

```
- above output, we can see the threads are running parallel. 