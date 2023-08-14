use std::error::Error;
use  std::io::{self,BufRead,stdin, Read};
use  std::net::{UdpSocket,SocketAddr};
use  std::{str, env};




fn get_stdin_data()->Result<String,Box<dyn Error>>{
    let mut buf=String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf)
}

#[tokio::main]
async fn main()->Result<(),Box<dyn Error>>{
    let remote_addr:SocketAddr=std::env::args()
                                   .nth(1)
                                   .unwrap_or_else(||"127.0.0.1:8080".into())
                                   .parse()?;
    let local_addr: SocketAddr=if remote_addr.is_ipv4(){
        "0.0.0.0:0"
    }else{
        "[::]:0"
    }
    .parse()?;

    let socket= UdpSocket::bind(local_addr)?;
    const  MAX_DATAGRAM_SIZE:usize=65_507;

    socket.connect(&remote_addr)?;
    
    let data=get_stdin_data()?;
    socket.send(&data.as_bytes())?;

    let mut data=vec![0u8;MAX_DATAGRAM_SIZE];
    let len=socket.recv(&mut data)?;

    println!("Received {} bytes: \n{}",len,String::from_utf8_lossy(&data[..len]));
    Ok(())
}