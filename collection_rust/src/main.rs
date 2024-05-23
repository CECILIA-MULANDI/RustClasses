// // fn main(){
// //     let mut v = Vec::new();
// //     v.push(1);
// //     v.push(2);
// //     v.push(3);

// //     for  i in 0..v.len(){
// //         v[i]+=1;
        
// //     }
// //     println!("{:?}",v);



// // }


// fn main(){
//     let date_time="2024-05-23T12:34:56";
//     let c:Vec<char>=date_time.chars().collect();

//     for i in 0..c.len(){
//         if c[i]=='T'{
//             println!("{}",&date_time[i+1..]);
//         }
//     }
// }




// struct with vec & hashmaps
// call fn to update vec and print
// fn to update hashmap and print


use std::collections::HashMap;
#[derive(Debug)]
struct Class{
    // name and grade
    student:HashMap<String,u32>
}
impl Class{
    fn new()->Class{
        Class{student:HashMap::new()}
    }
    fn update_student(&mut self,name:String,grade:u32){
        self.student.insert(name,grade);
    }
}

fn main(){
    let mut class1=Class::new();    
    class1.update_student("Mulandi".to_string(),98);
    class1.update_student("Mulandi".to_string(),99);
    println!("{:?}",class1);
}