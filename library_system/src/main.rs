mod books;
use books::Book;
mod area;
use area::Rectangle;
fn main() {
/* */
let res=Book::new("String theory".to_string(),1,"Big Bang theory".to_string());
    
    println!("{:?}",res.check_availability());
    let rec1=Rectangle::new(20,4);
    println!("The area of the rectangle is: {:?}",rec1.find_area());
}
