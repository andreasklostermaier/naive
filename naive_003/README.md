# Berlin Rust Hack and Learn: The NAIVE<sup>*</sup> Sessions

## 2024-04-11: NAIVE<sup>*</sup> session #003 – New Type Idiom

<sup>*</sup>Novice And Intermediate Vocational Exercises

### Challenge description

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

### Solutions/Ideas

(Solutions are presented as-is and are not necessarily functional)

#### 1. Combine the Newtypes into an enum which then implements the traits:

* https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7f49a0470cf692250bc4c3c92ae22616
* https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ddf8ca56713f4f4fef6f5a8f17594b5b
* https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c34074a9788c45fa69f0098df97bf5ca

Biggest downside: We actually want to treat `FooId` and `BarId` as types for different purposes.
Combining them into enums makes us lose the benefits of the type system.

#### 2. Use `macro_rules!` macros

* https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=73628611a25f3af36d0fea5788e7169e
* https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=dc5d7f1587f0af98250ffe05bf707dcf

#### 3. Extend the trait with a method that abstracts over the access to `.0`

* https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=9e3e0e21bc1bad74212dc281a44402da

Downside: Allocates when `into_string()` is called


#### 4. Use `std::mem::transmute` to expose the `String` directly

* https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=56f684682f55bdde733796e0518a2ca2

Don't try this at home. Funny solution though.

#### 5. Define a `derive` macro

Just an idea, but no implementation
