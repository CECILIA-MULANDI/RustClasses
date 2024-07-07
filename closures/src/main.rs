fn main() {
    // let multiple=|x:i32|->i32{
    //     x*3
    // };
    // let res=multiply_by_three(multiple);
    // println!("Result is {}",res)
    // let numbers = vec![1,2,3,4,5,6];

    // let res=numbers.iter().find(|x| **x>5).unwrap();
    // println!("The number greater than 5 is {:?}",res);
    #[derive(Debug)]
    struct Person{
        name:String,
        age:u32,
    }

    let person1=Person{name:"Keith".to_string(), age:35};
    let person2=Person{name:"Cecilia".to_string(),age:40};
    let mut vec1=Vec::new();
    vec1.push(person1);
    vec1.push(person2);

    let first=vec1.iter().find(|&p| p.age>30);
    println!("The first person whose age is greater than 30 is {:?}",first);
    
   
}
// fn multiply_by_three<K>(par_1:K)-    >i32 where K:Fn(i32)->i32{
//     par_1(10)
// }


// vec 