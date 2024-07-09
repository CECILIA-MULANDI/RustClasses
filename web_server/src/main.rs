use std::{
   
    fmt::format, io::{prelude::*,BufReader}, net::{TcpListener,TcpStream}
};
use std::fs;
fn main() {
    // create an instance of a server
    let listening= match TcpListener::bind("127.0.0.1:7878"){
        Ok(listening)=>listening,
        Err(e)=>{
            eprintln!("Failed to bind to address: {}", e);
            return;
        }

  };
    for stream in listening.incoming(){
       match stream{
        Ok(stream)=>{
            println!("Connection {:?}",stream);
            handle_con( stream)
        },
        Err(e)=>{
            eprintln!("Failed to establish a connection: {}", e);
        }
       }
    }
    
}
fn handle_con(mut stream:TcpStream){
    let mut buffer=[0;1024];
   stream.read(&mut buffer).unwrap();
//    println!("Request:{}",String::from_utf8_lossy(&buffer[..]))

    let req=b"GET / HTTP/1.1\r\n";
   
    let (status,file)=if buffer.starts_with(req) {
       ("HTTP/1.1 200 OK","index.html")

    }else{
        ("HTTP/1.1 404 Error","404.html")
        
    };
   
    let errors=fs::read_to_string(file).unwrap();
    let res=format!("{}r\nContent-Length: {}\r\n\r\n{}",
   status,
    errors.len(),
    errors
);
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
    


}
 