use std::fs;

fn main() {
    println!("{}",readFile(String::from("hello1.txt")));
    println!("{}",readFile(String::from("hello.txt")));
}

fn readFile(filePath:String)->String{
    let greeting_file_result = fs::read_to_string(filePath);
    match greeting_file_result {
        Ok(file_content) => {
            return file_content.to_string();
        },
        Err(error) => {
            return error.to_string();
        }
    }
}