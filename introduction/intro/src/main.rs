fn main() {
   let a:i8=-15;
   let b:u8=70;
   let c:i32=150;
   let d:u32=70;
   let e:i64=150;
   let f:u64=70;
   let g:i128=150;
   let i:u128=70;
   let j:isize=150000000000000;
   let k:usize=70;
  
   println!("{j}");
   println!("{k}");
//    floats    
    let f1=9.22;
    println!("{f1}");


   let  text_1=String::from("Welcome");
   let my_slice=&text_1[0..=5];
   println!("{}",my_slice);
   let mut s1=String::new();
   s1.push_str("See ");

   s1.push('m');
   s1.push('e');
   println!("{}",s1);

   let mut m1="Hey ".to_string();
  
   m1.push_str("there");
   println!("{}",m1);
   println!("{}",multiply(3,-2));


}



fn multiply(num1:i32,num2:i32)->i32{
    let mut pro=num1*num2 ;
    pro

}
