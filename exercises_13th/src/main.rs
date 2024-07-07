// two vectors 1..50
// 51..100
// hashmap ..first -key
// second ..second vector in descending order
// 1(key)..100(value)

// task 2
// shop mngt sys--stores items and quantities
// eg pen -10
// structs and traits
// traits to show a summary of the stock

use std::collections::HashMap;
#[derive(Debug)]
struct Shop{
    items:HashMap<String,u64>
}
trait ShopManagement{
    fn new(items:HashMap<String,u64>)->Self;
    fn add_item(&mut self,item:String,quantity:u64);
    fn summary(&self)->HashMap<String,u64>;
    fn get_count(&self)->usize;

}
impl ShopManagement for Shop{
    fn new(items:HashMap<String,u64>)->Self{
        Shop{items}
    }
    fn add_item(&mut self,item:String,quantity:u64){
        self.items.insert(item,quantity);
    }
    fn summary(&self)->HashMap<String,u64>{
        self.items.clone()
    }
    fn get_count(&self)->usize{
        self.items.len()
    }
       

    
}
fn main() {
   
    let mut map=HashMap::new();
    map.insert(String::from("Pen"),10);
    let mut shop = Shop::new(map);
    
    shop.add_item("Car".to_string(),20);
    
    println!("Below are the items in the shop {:?}",shop.summary());
    println!("The count of items is {:?}",shop.get_count());


    // let mut vec1=Vec::new();
    // let mut vec2=Vec::new();
    // for i in 1..=50{
    //     vec1.push(i);
    // }
    // for j in (51..=100).rev(){
    //     vec2.push(j);
    // }
    // println!("Vector one is :{:?}",vec1);
    // println!("Vector two is: {:?}",vec2);
    // // create a new hashmap
    // let mut map=HashMap::new();
    // for (key, value) in .zip(vec2.iter()) {
    //     map.insert(*key, *value);
    // }
    
    // println!("{:?}", map);
    

}
