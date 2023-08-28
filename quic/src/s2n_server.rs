use s2n_quic::{provider::connection_id,Server};
use std::{net::SocketAddr,path::Path, fs::File, io::BufReader};
use  tokio::sync::broadcast;
use  rustls::PrivateKey;

// const PATH_KEY:&str="./host.key";
// const PATH_CERT:&str="./host.pem";
pub static CERT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),"./host.pem"
));
/// NOTE: this certificate is to be used for demonstration purposes only!
pub static KEY: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),"./host.key"
));

#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{
  
 //let privatekey=load_private_key_from_file(PATH_KEY)?;
 //let publickey=load_private_key_from_file(PATH_CERT)?;
 //server
 let mut server=Server::builder()
                       .with_tls((CERT,KEY))?
                       .with_connection_id(connection_id::Default::default())?
                       .with_io("127.0.0.1:6666")?
                       .start()?;
        println!("Listening........[127.0.0.1:6666]");
 //let (tx,_)=broadcast::channel::<&[u8]>(128);

while let Some (mut conn)=server.accept().await{
    println!("Accepted the connected");

    tokio::spawn(async move{
        eprintln!("From: {:?}",conn.remote_addr());
        while let Ok(Some(mut stream)) = conn.accept_bidirectional_stream().await {
            println!("Openning .......");
            while let Ok(Some(data)) = stream.receive().await {
                stream.send(data).await.expect("stream should be open");
            }
        }
    });
}
 Ok(())   
}
fn load_private_key_from_file(path: &str) -> Result<PrivateKey, Box<dyn std::error::Error>> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut reader)?;

    match keys.len() {
        0 => Err(format!("No PKCS8-encoded private key found in {path}").into()),
        1 => Ok(PrivateKey(keys.remove(0))),
        _ => Err(format!("More than one PKCS8-encoded private key found in {path}").into()),
    }
}