// Create a program that takes in an input on the 
// terminal(hint: input should be an integer, print out from 0 
    // to the input(see a loop, for in and while loop))
use std::io::{self};
fn main() {
 print_to_n()   
}

fn print_to_n(){
    println!("Kindly enter a number");
    let mut  user_input=String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_n)=>{
           let num=user_input.trim().parse().expect("Not a number");
           if num>=0{
            for i in 0..=num{
                println!("{}",i);
            }


           }else{
            for i in (num..=0).rev(){
                println!("{}",i);
            }

           }
            

            
        }
        Err(e)=>println!("error {e}")
        
    }
    

}
