

fn main(){
    let val1 = find_first_a(String::from("prem"));
    // find_first_a(String::from("prem"));
    match val1{
        Some(val1)=> println!("first a index {}",val1),
        None =>{ println!("a not found");}
    }
}

fn find_first_a(s:String)->Option<i32>{
    for(index,char) in s.chars().enumerate(){
        if char =='a'{
            return Some(index as i32);
        }
    }
    return None;
}