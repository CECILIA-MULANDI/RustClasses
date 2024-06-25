// trait Multiplier{
//     fn mult(&self)->i32;
// }
// impl Multiplier for Vec<i32>{
//     fn mult(&self)->i32{
//         let mut ans=1;
//         for &i in self.iter(){
//             ans*=i
//         }
//         ans
        
//     }
// }

// fn print_res<T:Multiplier>(items:T){
//     println!("Product is {}",items.mult());
// }
// fn main() {
//     let numbers=vec![1,2,4];
//     print_res(numbers);
    
// }

mod news;
use news::newss::{News, Reporting};
mod area;
use area::areas::{Shape,FindArea};
fn main(){
    let news1=News::get_news("maandamano".to_string(),"Tuesday we go for maandamano".to_string(),3);
    println!("Today's news is {:?}",news1);
    println!("{:?}",news1.display_title());

    let rec=Shape{param1:2,param2:8};
    println!("the area of the rectangle is {:?}",rec.area());
    let circ=Shape{param1:2.1, param2:2.0};
    println!("The area of the circle is {:?}",circ.area())


}
