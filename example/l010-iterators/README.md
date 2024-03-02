# Iterators:

- used extensively in Rust for iterating over collections.
- `into_iter()`: consumes the collection and returns owned iterators.
- `iter()`: iterate over immutable references, preserving the collection.
- `iter_mut()`: iterates over mutable references, allowing modification.
- syntactic sugar: `for loop` directly on collection uses `into_iter()`.
    - `&` before collection in `for loop` uses `iter()`.
    - `&mut` before collection in `for loop` uses `iter_mut()`.

### iterator adapters:

- transform iterators before consumption.
- `map()`: apply a closure to each item in the iterator.
- `filter()`: keeps items based on a closure returning `true`.

### iterator consumers:

- perform action on the final iterator.
- `for_each()`: execute a closure for each item.
- `sum()`: calculate the sum of all items.
    - requires type annotation for generic collection.
    - use turbofish syntax (::<T>) to specify generic type.
- `collect()`: gathers items into a new collection.
    - requires type annotation for the collection type.
    - use underscore (_) to avoid specifying item type.

### additional points:

- turbofish syntax (::<T>) is used to specify generic type.
- `drain()` method empties a collection while preserving it.

