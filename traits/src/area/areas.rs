use std::{ops::Mul, process::Output};

#[derive(Debug)]
pub struct Shape<T>{
    pub param1:T,
    pub param2:T
}

pub trait FindArea<T>{
    fn area(&self)->T;

}
impl <T> FindArea<T> for Shape<T>where T:Mul<Output =T> +Copy,{
    fn area(&self)->T {
       self.param1* self.param2
    }
}