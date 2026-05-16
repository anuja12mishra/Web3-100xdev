## ENUM IN RUST
### Rust default enum's
Result enum: This is used for error handling.
    -> OK(T):give as the actual value of the result.
    -> Err(T):give as the error if any.

Option enum: This is used for Nullable value.
    -> Some(T): give us the value 
    -> None : it denote there is no resultn 

## Generic IN RUST
Generic is a feature that allows us to write code that can work with different types. It is a powerful tool that can help us to write more flexible and reusable code. In Rust, we can use generics with structs, enums, functions, and methods.
### Generic with Structs
We can define a struct with generic type parameters. For example, we can define a struct called Point that can hold any type of coordinates:

```rust
struct Point<T> {
    x: T,
    y: T,
}
```
In this example, T is a generic type parameter that can be replaced with any type when we create an instance of the Point struct. For example, we can create a Point with integer coordinates:

```rust
let p1 = Point { x: 5, y: 10 };
```
Or we can create a Point with floating-point coordinates:

```rust
let p2 = Point { x: 1.5, y: 2.5 };
```
### Generic with Enums
We can also define enums with generic type parameters. For example, we can define an enum called
Option that can hold any type of value:

```rust
enum Option<T> {
    Some(T),
    None,
}
```
In this example, T is a generic type parameter that can be replaced with any type when we create an instance of the Option enum. For example, we can create an Option with an integer value:

```rust
let o1 = Option::Some(5);
```
Or we can create an Option with a string value:

```rust
let o2 = Option::Some(String::from("Hello"));
```
### Generic with Functions
We can also define functions with generic type parameters. For example, we can define a function called
swap that can swap the values of two variables of any type:

```rust
fn swap<T>(a: &mut T, b: &mut T) {
    let temp = a;
    *a = *b;
    *b = temp;
}
```
In this example, T is a generic type parameter that can be replaced with any type when we call the swap function. For example, we can swap two integers:

```rust
let mut x = 5;
let mut y = 10;
swap(&mut x, &mut y);
```
Or we can swap two strings:

```rust
let mut s1 = String::from("Hello");
let mut s2 = String::from("World");
swap(&mut s1, &mut s2);
```
### Generic with Methods
We can also define methods with generic type parameters. For example, we can define a method called
distance that can calculate the distance between two points of any type:

```rust
impl<T> Point<T> {
    fn distance(&self, other: &Point<T>) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
```
In this example, T is a generic type parameter that can be replaced with any type when we call the distance method. For example, we can calculate the distance between two points with integer coordinates:

```rust
let p1 = Point { x: 5, y: 10 };
let p2 = Point { x: 1, y: 2 };
let d = p1.distance(&p2);
```
Or we can calculate the distance between two points with floating-point coordinates:

```rust
let p1 = Point { x: 1.5, y: 2.5 };
let p2 = Point { x: 3.0, y: 4.0 };
let d = p1.distance(&p2);
```
In conclusion, generics in Rust allow us to write code that can work with different types, making our code more flexible and reusable. We can use generics with structs, enums, functions, and methods to create powerful abstractions that can handle a wide variety of data types.