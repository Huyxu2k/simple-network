use std::{net::{SocketAddr, UdpSocket}, error::Error};

//
struct Server {
    socket: UdpSocket,
    buf: Vec<u8>, //u16
    to_send: Option<(usize, SocketAddr)>,
}
impl Server {
    async  fn run(self) -> Result<(), std::io::Error> {
        let Server {
            socket,
            mut buf,
            mut to_send,
        } = self;

        loop {
            if let Some((size, peer)) = to_send {
                let amt = socket.send_to(&buf[..size], &peer)?;

                println!("Echoed {}/{} bytes to {}", amt, size, peer);
            }
            to_send = Some(socket.recv_from(&mut buf)?);
        }
    }
}

#[tokio::main]
async fn main() ->Result<(),Box<dyn Error>> {
    let addr=std::env::args()
                      .nth(1)
                      .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let socket=UdpSocket::bind(&addr)?;
    println!("Listening on: {}",socket.local_addr()?);

    let server=Server{
        socket,
        buf:vec![0;2048],
        to_send:None
    };
    server.run().await?;
    Ok(())
}
