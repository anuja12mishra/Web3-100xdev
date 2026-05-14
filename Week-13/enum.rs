#[derive(PartialEq)]
enum DIRECTION{
    north,
    south,
    west,
    east
}
fn main(){
    move_away(DIRECTION::north);
    move_away(DIRECTION::south);
}
fn move_away(direction:DIRECTION){
    if(direction  == DIRECTION::north){
        println!("move north")
    }
    if(direction  == DIRECTION::west){
        println!("move west")
    }
    if(direction  == DIRECTION::south){
        println!("move south")
    }
    if(direction  == DIRECTION::east){
        println!("move east")
    }
}