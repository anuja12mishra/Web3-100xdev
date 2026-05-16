use std::fs;

fn main() {
    let greeting_file_result = fs::read_to_string("hello.txt");
    // print!("{}", greeting_file_result.unwrap());
    print!("{}", greeting_file_result.unwrap_or(String::from("Empty or File does't exist")));
}