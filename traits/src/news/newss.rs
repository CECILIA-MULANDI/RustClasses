#[derive(Debug)]
pub struct News{
    pub name:String,
    headlines:String,
    number:u32
}

pub trait Reporting{
     fn get_news(name:String,headlines:String,number:u32)->Self;
     fn display_title(&self)->String;
       
}

impl Reporting for News {
    fn get_news(name:String,headlines:String,number:u32)->Self{
        News{name,headlines,number}

    }
    fn display_title(&self)->String{
        format!("{}",self.name)
    }
}