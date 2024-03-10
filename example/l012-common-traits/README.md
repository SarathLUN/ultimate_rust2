# Lesson 12: Common Traits

1. Derivable traits: these traits can be automatically implemented using the `derive` attribute for structs and enums. Examples include `Debug` for pretty-printing and `Clone` for copying values.
2. Manually implementing traits: this involves three steps: importing the trait, using boilerplate code (often provided by IDE), and implementing the required methods.
3. `Default` trait: allows you to define default value for a struct.
4. `PartialEq` and `Eq` traits: checks for equality, while `Eq` is a marker trait for types with fully transitive, symmetric, and reflexive equality.
5. `From` and `Into` trait: `From<U>` allows conversion from type `U` to the current type, and `Into<T>` allows conversion from the current type to type `T`. Implementing `From` automatically provides `Into`.

