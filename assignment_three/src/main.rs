//  Create a program that takes in an input from the terminal 
// and subtract, addition, multiply and division (any number)

use std::io;


fn main() {
    calculator();
}

fn calculator(){
    println!("Enter num1");
    let mut user_input1=String::new();
    io::stdin().read_line(&mut user_input1).expect("Could not read the line");
    let num1:i32=user_input1.trim().parse().expect("Not a number");


    println!("Enter num2");
    let mut user_input2=String::new();
    io::stdin().read_line(&mut user_input2).expect("Could not read the line");
    let num2:i32=user_input2.trim().parse().expect("Not a number");
    
    println!("{num1}, {num2}");

    let sum = num1+num2;
    let difference = num1-num2;
    let multiplication = num1*num2;
    let division =if num2 !=0{
        Some(num1/num2)

    }else{
        None

    };

    println!("Addition: {} + {} = {}", num1, num2, sum);
    println!("Subtraction: {} - {} = {}", num1, num2, difference);
    println!("Multiplication: {} * {} = {}", num1, num2, multiplication);
    match division {
        Some(q) => println!("Division: {} / {} = {}", num1, num2, q),
        None => println!("Division: Division by zero is not allowed"),
    }

    
    

}
