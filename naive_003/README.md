# String Identifiers as NewTypes

Rust supports the [NewType Idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).
You can use it to wrap an existing type with another type that has a narrower semantic meaning.
For this excercise, we are looking at NewTypes, that wrap `String`s. Each one of our structs looks like this:

```rust
struct MyId(String);
```

And we need a way to construct those structs.
We do this by implementing the [FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html) trait,
which let's us do the following:

```rust
let id: MyId = "some string slice that is an ID value".parse()?;
```

However, we also might want to use the `String` and `&str` methods on the wrapped type.
For this we define our own trait `StringIdentifier`.
This trait has a method `as_str(&self) -> &str` and a method `into_string(self) -> String`,
to borrow a string slice and to extract the underlying `String` respectively.

Challenge: Look at the code in the `src/lib.rs` file. It has a lot of repetition. How can we improve it?

Restriction: [DerefMut](https://doc.rust-lang.org/std/ops/trait.DerefMut.html)
and other mechanism that can be used to bypass the `FromStr` parsing constraints are not allowed.
