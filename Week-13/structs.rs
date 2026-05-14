// struct address{
//     city:String,
//     pincode:String
// }

struct User{
    name:String,
    age:u32,
    // address:address  
}

impl User{
    fn is_allowed_to_vote(&self)->bool{
        if self.age >= 18 {
            return true
        } return false;
    }
}

fn main(){
    let u1:User = User{
        name:String::from("anuja"),
        age:23,
        // address: Vec![]
    };
    let u2:User = User{
        name:String::from("archi"),
        age:12,
        // address: Vec![]
    };
    println!("{}",u1.is_allowed_to_vote());
    // println!("{}",is_allowed_to_vote(&u2));
    // println!("{:?}",u2);
}

