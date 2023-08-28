use  s2n_quic::{client::Connect,provider::{self,tls},Client};
use std::{net::SocketAddr,path::Path,time::Duration,io::{Read,Write,stdin,stdout}};
use tokio::io::AsyncWriteExt;

pub static CERT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),"./host.pem"
));

#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{
    
    let time=Duration::new(1,0);
    //client
    let mut client=Client::builder()
                 .with_tls(CERT)?
                 .with_io("0.0.0.0:0")?
                //  .with_limits(
                //     provider::limits::Limits::new().with_max_handshake_duration(time).unwrap(),
                //  )?
                 .start()?;
    let addr:SocketAddr="127.0.0.1:6666".parse()?;
    let connect= Connect::new(addr).with_server_name("local");
    let mut connection=client.connect(connect).await?;
    
    connection.keep_alive(true)?;

    while let Ok(mut stream)=connection.open_bidirectional_stream().await {
        loop {
            let i=input!("spm:");
            if i.trim()=="q"{
                break;
            }
            stream.write(i.as_bytes()).await?;
        }
    }
    connection.close(0u8.into());
    Ok(())
}


#[macro_export]
macro_rules! input {
    () => {
        {
            let mut input=String::new();
             stdin()
               .read_line(&mut input)
               .expect("failed to read");
            input
        }
    };
    ($x:expr)=>{
        {
            {
                let mut input =String::new();
                print!("{}:",$x);
                stdout().flush().unwrap();
                stdin()
                     .read_line(&mut input)
                     .expect("failed to read line");
                input
            }
        }
    };
}