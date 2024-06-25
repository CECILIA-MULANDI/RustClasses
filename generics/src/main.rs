#[derive(Debug)]
struct User<D>{
    username:D,
    location:D
}
impl <D> User<D>{
    fn new(username:D,location:D)->Self{
        User{username,location}
    }
    fn get_user(&self)->&User<D>{
        &self
    }
}
fn main() {

    let new_user=User::new("Cecilia".to_string(),"Kenya".to_string());
    println!("The information for is {:?}  ",new_user.get_user());
    // let numbers = [1,10,90,900];
    // let res=largest_integer(&numbers);
    // println!("the largest number is {}",res);

    // let characters=['x','c','y'];
    // let biggest=find_largest(&characters);
    // println!("the largest character is {}",biggest);
}


 