use  std::{error::Error,net::SocketAddr};

use quinn::Endpoint;
mod common;


use  common::{make_client_endpoint,make_server_endpoint};


#[tokio::main]
async fn main()->Result<(),Box<dyn Error>>
{
  let addr1="127.0.0.1:5000".parse().unwrap();
  let addr2="127.0.0.1:5001".parse().unwrap();
  let addr3="127.0.0.1:5002".parse().unwrap();
  let server1=run_server(addr1)?;
  let server2=run_server(addr2)?;
  let server3=run_server(addr3)?;
  

  let client=make_client_endpoint("127.0.0.1:0".parse().unwrap(), &[&server1, &server2, &server3])?;

  tokio::join!(
    run_client(&client,addr1),
    run_client(&client,addr2),
    run_client(&client,addr3),
  );

  client.wait_idle().await;
  Ok(())
}

fn run_server(addr:SocketAddr)->Result<Vec<u8>,Box<dyn Error>>{
   let (endpoint,server)=make_server_endpoint(addr)?;
   
   tokio::spawn(async move{
     let connection=endpoint.accept().await.unwrap();
     println!("[SERVER] incoming connection: address={}",connection.remote_address());
   });

   Ok(server)
}
async fn run_client(endpoint: &Endpoint, server_addr: SocketAddr) {
  let connect = endpoint.connect(server_addr, "localhost").unwrap();
  let connection = connect.await.unwrap();
  println!("[CLIENT] connected: addr={}", connection.remote_address());
}