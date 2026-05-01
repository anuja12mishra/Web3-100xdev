fn main() {
    // let a: i64 = 233434343;
    // let x: i32 = 1;
    // let z: i8 = 10;
    // let _x : f32 = 34.3;
    // let b:bool = false;
    // println!("{}",b);
    vector();
}

fn vector(){
    let mut xs = vec!();
    
    println!("{}", xs.len());

    xs.push(4);
    
    println!("{:?}", xs);
}