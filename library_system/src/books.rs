
#[derive(Debug)]
pub struct Book{
    name:String,
    status:u32,
    category:String
}

impl Book{
    
    pub fn new(name:String,status:u32,category:String )->Self{
       return Book{name,status,category}

    }
   pub fn check_availability(&self)->bool{
   self.status==1
}
}


