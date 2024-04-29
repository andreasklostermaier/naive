# Berlin Rust Hack and Learn: The NAIVE<sup>*</sup> Sessions

## 2024-03-14: NAIVE<sup>*</sup> session #004 – String concatenation

<sup>*</sup>Novice And Intermediate Vocational Exercises

### Challenge description

We start from the code in src/main.rs:

We want to create and print a variable **pb** of type **PathBuf**. The PathBuf type is imported in line 1 from the standard library. The path **/home/naive/session/session_42/README.txt** has to be constructed **in an OS agnostic way** from the provided variables a to e (the variables must be used, but new derived variables are allowed). We also define another important constraint: the use of the **format!** macro is not allowed anywhere in the code.

This task would be extremely easy to solve in highlevel languages like Javascript, but can become quite complicated in Rust due to type safety constraints - at least for Rust beginners.


### Solutions

During the session we came up with four approaches:

#### 1. Iterate

The non-String type variables are converted with the `to_string()` function, the session-subfolder is concatenated with `push_str()` and the path is constructed by iterating over path segments into the PathBuf type.

```bash
cargo run --example iterator
```

#### 2. Macro

Just for the fun of it, we have a Macro solution, which iterates over its arguments and collects into the PathBuf. The session subfolder name is constructed conventionally.

```bash
cargo run --example macro
```


#### 3. Pushing

This is a very conventional way, where everything is converted to string where necessary and then pushed together. Despite its simplicity, there is an interesting detail: variable `result` is created as PathBuf, while variable `dir`is created as String. In `dir` we concatenate the elements of the session subfolder name with `.push()` for the char and `push_str` for the integer. The `.push()` calls to `result` provide the path delimiters!

```bash
cargo run --example push
```

#### 4. Arbitrary conversions

This is a very simple and short solution, but conversions are sort of messy. We convert string literal `b` to String, so we can `.push()` char `c` and Integer `d`, which we have to convert to String first and then take as slice. Finally, within the PathBuf constructor, we have to cast some variables back to `str`.

```bash
cargo run --example messy
```

### Lessions learned on the way

In all of the examples we used PathBuf invocations that autocreate the path delimiters. That way PathBuf takes care of operating system compatibility and we didn't have to provide path delimiter characters on our own.