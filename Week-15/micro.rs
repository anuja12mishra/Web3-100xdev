use std::fmt::{Display};

// #[derive(Debug)]
struct MyStruct
{
    field1: i32,
    field2: String,
}
impl Display for MyStruct
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "MyStruct {{ field1: {}, field2: {} }}", self.field1, self.field2)
    }
}
fn main()
{
    let s = String::from("Hello, world!");
    let my_struct = MyStruct { field1: 42, field2: String::from("Hello, world!") };
    println!("{}", my_struct);
    println!("{}", s);
}
