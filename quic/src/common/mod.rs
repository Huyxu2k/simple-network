//#![cfg(feature="rustls")]
use quinn::{ClientConfig,Endpoint,ServerConfig};
use  std::{error::Error,net::SocketAddr,sync::Arc};


#[allow(unused)]
pub fn make_client_endpoint(bind_addr:SocketAddr,sercer_certs:&[&[u8]])->Result<Endpoint,Box<dyn Error>>{
   let client_cfg=configure_client(sercer_certs)?;
   let mut endpoint=Endpoint::client(bind_addr)?;
   endpoint.set_default_client_config(client_cfg);
   Ok(endpoint)
}
fn configure_client(server_certs:&[&[u8]])->Result<ClientConfig,Box<dyn Error>>{
    let mut certs: rustls::RootCertStore=rustls::RootCertStore::empty();
    for cert in server_certs {
        certs.add(&rustls::Certificate(cert.to_vec()))?;
    }
    let client_config=ClientConfig::with_root_certificates(certs);
    Ok(client_config)
}
#[allow(unused)]
pub fn make_server_endpoint(bind_addr:SocketAddr)->Result<(Endpoint,Vec<u8>),Box<dyn Error>>{
   let (server_config,server_cert)=configure_server()?;
   let endpoint=Endpoint::server(server_config,bind_addr)?;
   Ok((endpoint,server_cert))

}
fn configure_server()->Result<(ServerConfig,Vec<u8>),Box<dyn Error>>{
   let cert=rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
   let cert_der= cert.serialize_der().unwrap();
   let priv_key=cert.serialize_private_key_der();
   let priv_key=rustls::PrivateKey(priv_key);
   let cert_chain=vec![rustls::Certificate(cert_der.clone())];

   let mut server_config=ServerConfig::with_single_cert(cert_chain, priv_key)?;
   let transport_config=Arc::get_mut(&mut server_config.transport).unwrap();
   transport_config.max_concurrent_uni_streams(0_u8.into());
  
   Ok((server_config,cert_der))
}

#[allow(unused)]
pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];

