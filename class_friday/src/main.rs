struct User{
    name:String,
    age:i32,
    id_no:i32,
}

impl User{
    pub fn walk(&self){
        println!("User {} is walking",&self.name)
    }
    pub fn eat(&self){
        println!("User {} is eating ",&self.name)
    }
    pub fn is_adult(&self)->bool{
        if self.age>18{
            return true;
        }
        return false;
    }
}

fn main(){
    let person = User{
        name:String::from("Mulandi"),
        age:21,
        id_no:90000,
    };
    let res = person.is_adult();
    println!("Are an adult? {res}");
}