use std::thread;
use  std::net::{TcpListener,TcpStream,Shutdown};
use  std::io::{Read,Write};


fn handle_client(mut stream:TcpStream){
    let mut data=[0 as u8;100];
    while match stream.read(&mut data) {
        Ok(size)=>{
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_)=>{
            println!("An error occurred,terminating connection with {}",stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main(){
     let listener= TcpListener::bind("0.0.0.0:3001").unwrap();

     println!("Server listening on port 3001");
     for stream in listener.incoming() {
         match stream {
             Ok(stream)=>{
                println!("New connection: {}",stream.peer_addr().unwrap());
                thread::spawn(move||{
                    handle_client(stream)
                });
             }
             Err(e)=>{
                println!("Error: {}",e);
             }
         }
     }
     drop(listener);
}