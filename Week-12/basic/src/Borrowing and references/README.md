# Borrowing and references

Rather than transferring ownership to a function, you can let the function `borrow` the variable

```rust
fn main() {
    let str = String::from("Harkirat");
    let len = get_length(&str);
    println!("{} {}", str, len);
}

fn get_length(str: &String) -> usize {
    let len = str.len();
    return len
}
```

When you pass a variable by reference, the variable is still owned by the first function. It is only borrowed by the `get_length` function.

### Rules of borrowing -

1. You can only have one immutable reference. If there is an immutable reference, there cant be other immutable or mutable references
2. You can have multiple immutable references

### Quiz

1. Will this code compile

```rust
fn main() {
    let str = String::from("Harkirat");
    let ref1 = &str;
    let ref2 = &str;

    println!("{} {}", ref1, ref2);
}

```

1. Will this code compile

```rust
fn main() {
    let mut str = String::from("Harkirat");
    let ref1 = &mut str;
    let ref2 = &str;
    
    println!("{} {}", ref1, ref2);
}

```

1. Will this code compile

```rust
fn main() {
    let mut str = String::from("Harkirat");
    let ref1 = &mut str;
    ref1.push_str("Singh");
    let ref2 = &str;
    
    println!("{}", ref2);
}
```

## Assignments

### 1. Borrowing an Immutable Reference

**Goal:** Write a function `calculate_length` that takes an **immutable reference** to a `String` and returns its length. Then call this function from `main` and print both the original `String` and its length.

- Hint
    
    ```rust
    fn calculate_length(s: &String) -> usize {
        // ...
    }
    ```
    

### 2. Borrowing a Mutable Reference

**Goal:** Write a function `append_text` that takes a **mutable reference** to a `String` and appends some text to it. For example, if the string is `"Hello"`, the function could append `", World!"`.

- Hint
    
    ```rust
    fn append_text(s: &mut String) {
        // ...
    }
    ```