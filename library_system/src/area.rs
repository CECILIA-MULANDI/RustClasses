pub struct Rectangle{
    height:u32,
    width:u32,
}


impl Rectangle{
    pub fn new(height:u32,width:u32)->Self{
       Rectangle{height,width} 
    }
    pub fn find_area(&self)->u32{
        self.height*self.width
    }
}