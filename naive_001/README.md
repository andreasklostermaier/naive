# Berlin Rust Hack and Learn: The NAIVE<sup>*</sup> Sessions

## 2024-02-15: NAIVE<sup>*</sup> session #001 – Metaprogramming

<sup>*</sup>Novice And Intermediate Vocational Exercises

### Challenge description

We start from the code in src/main.rs:

In the global scope we define a struct **MyData** with 10 fields, each
of type String (in the real world, the struct might, for example, represent a 
database table and the struct fields might represent its columns).

```rust
struct MyData {
    a: String,
    b: String,
    c: String,
    d: String,
    e: String,
    f: String,
    g: String,
    h: String,
    i: String,
    j: String,
}
```

Some of the fields need to be converted to uppercase (in the real
world scenario certain columns maybe need to be encrypted before
insertion in a database). This is done in the function **string_conversion**.

Applying the conversion on a selection of fields of MyData leads to repetitive code lines as
seen in the main function between the two println! statements. There is nothing wrong with that per se, there is no penalty in performance or memory. It is merely an aesthetical thing that those repetive lines clutter the source code and mean a lot of writing effort.

The challenge of this excercise is to avoid the code repitition of the
**string_conversion()** statements. We would rather like to have something like a wrapper function 
and pass in the field names that require the conversion as an array. That way we
would clean up the code and we would have a generic solution for the conversion of 
arbitrary field-selections in other contexts. And we want as little abstraction cost as possible.

But field names are internal symbols, not typed values. There is no obvious way to address a struct field name with a String, str or other type. There is also no obvious way to iterate over struct fields. This is a typical problem of **metaprogramming**: the source code is not part of the processed data.


### Solutions

During the session we came up with four approaches:

#### 1. Use of Macros

Macros come to mind immediately: by definition they produce source code (in Rust's case on compile time) from a template and Macros are first class citizens in Rust. Writing a Macro is THE obvious approach, BUT: Macros are a little difficult, in Rust maybe more so than in other languages. Most of the participants in our workshop had no prior experience with Rust Macros, but one or two of them (thanks Craig!) quickly came up with a working solution as shown in example **macro.rs**:

```bash
cargo run --example macro
```

There are no penalties: the compiled code is exactly as efficient as the original code. We'd consider this a really good solution to the problem.

#### 2. Passing References

In this solution we pass a Vector of references to the affected struct fields to our string_conversion function. We then iterate over the references and pass each element to a closure, which does the uppercase conversion into an owned local variable and "pokes" the result back into the original String object (on the heap) by derefencering the reference with the **\*** operator:

```bash
cargo run --example reference
```

This solution looks pretty legit. Certainly not Zero Cost (at least we have a temporary variable that holds the converted string), but not bad. Closures and the dereferencing operator might be a little intimidating for Rust newbies coming from Highlevel languages. And we have not improved our source code appearance too much, as building the vector with references still looks a bit cluttered.

Note: we could not avoid the temporary variable in line 50, as we cannot write to the element while we read from it. Alternative line 50 is NOT possible:

```rust
*element = element.to_uppercase();
```

#### 3. Trait implementation

One of the participants (thanks llogiq!) came up with another very interesting and Rusty solution: let the field convert itself upon initialization!

For that purpose we created a designated type **MyConvertedString** and assigned that type to those fields of the **MyData** struct definition that need to be converted. We then implemented the **From** Trait from sdt::convert for **MyConvertedString**, by defining a **from** function that does the uppercase conversion. By implementing the **From** Trait we get the **into()** method for free. The **string_conversion** function can be removed and we do not need to println! before and after the initialization: everything is done after **my_string** has come to life:

```bash
cargo run --example trait
```

This approach has some interesting side effects. For one, the designated type for converted strings makes sure, that those fields actually contain only converted values upon initialization and it prevents us from erroneously double converting those fields. Nice! This is probably the best and most idiomatic solution for our little challenge.

One caveat: in our assumed real world scenario, if the MyData struct resembles a database table and ORM is involved, there might be an automatic mapping between database column types and the Rust types (e.g. String => nvarchar). In that case extra work might be necessary to provide compatibility.

Notes: 

- we have to import the From Trait with a use statement (line 1).
- we can assign the converted MyData fields either with the from-syntax (`MyConvertedString::from`) or with the into-syntax (`"some_string".into()`, see example at line 40). Both do exactly the same, the former is more expressive and was preferred by most of our participants for that reason, while the latter produces extremely lean source code.


#### 4. Serialization/Deserialization

We only discussed this approach, but did not code it. The idea is to serialize the struct to something that we can operate on at runtime, e.g. a JSON object or JSON string. We could then do the conversion and replace the element within the JSON object (by whatever means) and finally transform the JSON object back into a MyData object. The SERDE crate would have all the tools required, but this seems to be overly complicated and comes with quite some penalties, not least the requirement to embedd a large library.


### Lessions learned on the way

- Rust Macros can easily work with a flexible number of arguments.
- There are two kinds of Macros in Rust: procedural and declarative.
- Macro expansion can be visualized in **Rust Playground**, under "TOOLS > Expand macros"
- Decorating the struct definition and main function with #[allow(dead_code)] prevents clippy complaining about the struct fields that we didn't deal with in the excercise.
- The Strings of our **MyData** struct are not stored/packed in the struct itself! Each string is an independent Box in the Heap. The struct only contains the (fat) pointers to the strings. Understanding this makes the second solution (reference) much more comprehensible.