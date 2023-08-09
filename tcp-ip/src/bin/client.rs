use  std::net::TcpStream;
use  std::io::{Read,Write};
use std::str::from_utf8;



fn main(){
   match TcpStream::connect("localhost:3001") {
     Ok(mut stream)=>{
         println!("Success conneted to server in port 3001");

         let msg=b"Hello!";

         stream.write(msg).unwrap();
         println!("Sent hello, awaiting reply ....");

         let mut data=[0 as u8;6];

         match  stream.read_exact(&mut data) {
             Ok(_)=>{
                if &data==msg{
                    println!("Reply is ok !");
                }
                else {
                    let text=from_utf8(&data).unwrap();
                    println!("Unexpected reply :{}",text);

                }
             }
             Err(e)=>{
                println!("Failed to receive data :{}",e);
             }
         }
     }
     Err(e)=>{
        println!("Failed to connect :{}",e);
     }
   }
   println!("Terminated.");
}