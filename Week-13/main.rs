fn main(){
    let s:String = String::from("hello");
    let (s,res) = is_longer_than(s,3);
    println!("{}",res);
    println!("{}",s);
}

fn is_longer_than(s:String,num:usize)->(String,bool){
    if s.len() >num {
        return (s,true);
    }
    else{
        return (s,false);
    }
}