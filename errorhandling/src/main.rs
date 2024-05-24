// fn main() {
//    let my_string="Hello World".to_string();
//    let res = match my_string.chars().nth(11){
//     Some(c)=>c.to_string(),
//     None=>"NO characters at index 10".to_string()
//    };
//    println!("Character is {}",res);

//    let result=print_occ("Scientist");
// //    println!("{}",result.unwrap());
//    println!("{}",result.expect("This option is not available"));
// }
// fn print_occ(name:&str)->Option<&str>{
//     match name{
//         "programmer"=>Some("Hey coder"),
//         "Mathematician"=>Some("Hey Mothermatician"),
//         _=>None
//     }
// }




fn main(){
    let x=take_user(vec![2,8],1);
    // println!("{:?}",x.expect("There was an error!!"));

    match x{
        Some(x)=>println!("{:?}",x),
        None=>println!("Error")
    };
}

fn take_user(a:Vec<u32>,b:usize)->Option<u32>{
    if a.len()>b as usize{
        Some(a[b])

    }else{
        return None
    }

}
