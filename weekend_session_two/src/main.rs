// // fn main() {
// //     let v=vec![1,2,3,4,5,6,7,8,9,10];
// //     // let mut vecs=Vec::new();
// //     // let vec2=&v[6..9];

// //     for i in 0..v.len(){
// //         if i==6 && i<v.len(){
            
// //             // vecs.push(v[i]);
// //             // vecs.push(v[i+1]);
// //             // vecs.push(v[i+2]);

// //             // println!("{:?}",vecs);
// //             println!("{:?}",&v[i..(v.len()-1)]);
// //         }
// //     }

// // }
// /**First */
// // traffic-light--red,green,orange
// // match==>print green- go,
// // orange-get ready to go
// // red--don't go
// // #[derive(Debug)]
// // enum Traffic{
// //     Red(String),
// //     Orange(String),
// //     Green(String)
// // }
// // fn call_traffic(traffic:Traffic)->String{
// //         match traffic{
// //             Traffic::Red(x)=>"Do not go!".to_string(),
// //             Traffic::Orange(x)=>"Get ready!:(".to_string(),
// //             Traffic::Green(x)=>"Go!".to_string(),
// //             _=>"The Road is blocked".to_string(),
// //         }
// // }



// // fn main(){
// //     let car1=Traffic::Red("x".to_string());
// //     println!("{:?}",call_traffic(car1)); 
// // }

// /**second */
// // struct Traffic{
// //     light:Lights,
// // }

// // enum Lights{
// //    Red,
// //    Green,
// //     Orange, 
// // }

// //  impl Traffic{
// //   fn call_light(&self){
// //         match self.light{
// //             Lights::Red=>println!("Don't Go"),
// //             Lights::Green=>println!("Go!"),
// //             Lights::Orange=>println!("Get ready"),
// //             _=>println!("We don't know")

// //         }

// //     }
// // }


// // fn main(){
// //     let t1=Traffic{light:Lights::Red};
// //     t1.call_light();
// // }

// // the lazy fox jumped the bridge lazy

// use std::collections::HashMap;
// use rand::Rng;
// fn main(){
//     // count_words()
//     generate_randoms()
// }
// // fn count_words(){
// //     let text="the lazy fox jumped the bridge lazy";
// //     // let v:Vec<char>=text.chars().collect();
// //     // println!("{:?}",v);

// //     let mut maps=HashMap::new();

// //      for i in text.split_whitespace(){
// //         let count=maps.entry(i).or_insert(0);
// //         *count+=1;
// //      }
// //      println!("{:?}",maps);


// // }

// // rand and generate 10 random numbers and orders descending


// fn generate_randoms(){
//     let mut v=Vec::new();
//     let mut v2=Vec::new();
//     let mut count=0;
//     while count<10{
//         let x=rand::thread_rng().gen_range(1..=100);
//         v.push(x);

        
//         count+=1;
// }
//     println!("{:?}",v);
//     // v.sort();
//     // v.reverse();
//     // println!("{:?}",v);

//     for i in 0..(v.len() - 1) {
//         for j in (i + 1)..v.len() {
//             if v[i] < v[j] {
//                 v2.push(v[j]);
//             } else {
//                 v2.push(v[i]);
//             }
//         }
        
//     }
//     println!("v2 is {:?}", v2);


       


   
    
   
   
    
   

// }





// Write a Rust function that takes a vector of tuples (i32, i32) and returns the sum 
// of all the first elements if the second elements are all even, 
// the sum of all the second elements 
// if the first elements are all odd, and 0 otherwise.
fn main(){
    summation((3,2),(9,8));
}
fn summation(a:(i32,i32),b:(i32,i32)){

    
    let v2=vec![a,b];

    for (item1, _) in v2{
        println!("item 1 {}",item1);
    }
    // println!("{:?}",v2);

    // if v2[a.1] && v2[b.1] % 2==0{
    //    let sum1= v2[a.0]+v2[b.0];
    //    println!("{sum1}");
    // }
}