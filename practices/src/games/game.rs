pub struct Character{
    name:String,
    score:u32,
    level:u32
}

impl Character{
    pub fn new(name:String,score:u32,level:u32)-> Character{
        Character{name,score,level}
    }
    pub fn new_level(&mut self)->u32{
        let next =self.level+1;
        next

    }
    
}