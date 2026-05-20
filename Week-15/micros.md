## micros in rust
```rust
fn main() {
    let s = String::from("Hello, world!");
    println!("{}", s);
}
```
### micros are of two types:
1. **Function-like macros**: These macros look like function calls but are expanded at compile time. They can take arguments and generate code based on those arguments. For example, the `println!` macro is a function-like macro that takes a format string and arguments to print to the console.
```rust
println!("Hello, {}!", "world");
```
2. **Attribute-like macros**: These macros are used to annotate items such as functions, structs, or modules. They can modify the behavior of the annotated item or generate additional code. For example, the `#[route]` macro is an attribute-like macro that can be used to define routes in a web application.

```rust 
#[route("GET", "/")]
fn index() -> &'static str {
    "Hello, world!"
}
```

- declatative micros are defined using the `macro_rules!` macro, which allows you to create macros that can match different patterns and generate code accordingly. For example, you can define a simple macro to create a vector of integers:

```rust
macro_rules! vec_of_ints {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

- procdual micros are defined using the `proc_macro` crate, which allows you to create more complex macros that can manipulate the syntax tree of the code. These macros can be used to generate code based on the input they receive. For example, you can define a procedural macro to create a new struct with specified fields:

```rust
use proc_macro::TokenStream;
#[proc_macro]
pub fn create_struct(input: TokenStream) -> TokenStream {
    // Code to parse the input and generate a new struct
}
```

