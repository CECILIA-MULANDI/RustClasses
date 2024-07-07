use num::traits::Zero;
use std::fmt::Display;
use std::ops::Div;
pub fn divide<A>(a: A, b: A) -> Result<A, String>where  A:Zero+Div<Output=A>+PartialEq+Display,{
    if a == A::zero() || b == A::zero() {
        return Err("You cannot divide by zero".to_string());
    }
    let  res= a/b;
    return Ok(res)

}
// pub struct Persons{
//     name:String,
//     age:u32,
// }
// impl Persons{
//     pub fn new_struct(name:String,age:u32)->Self{
//         Persons{name,age}
//     }
    
// }
// pub fn push_to_vec(vec:&mut Vec<Persons>,person:Persons )->usize{
    
//     vec.push(person);
//     let length=vec.len();
//     length

// }

#[cfg(test)]
mod tests {
    // imports everything outside the module
    use super::*;

    #[test]
    fn if_divides(){
        let testing=divide(4.0, 2.0);
        assert_eq!(testing,Ok(2.0));
    }
    // fn it_works() {
    //     let pers1=Persons::new_struct("Mulandi".to_string(), 8);
    //     let pers2=Persons::new_struct("Keith".to_string(), 10);
    //     let mut vec = Vec::new();
    //     let mut result = push_to_vec(&mut vec, pers1);
    //     result=push_to_vec(&mut vec,pers2);
    //     assert_eq!(result, 2);
    // }
}
