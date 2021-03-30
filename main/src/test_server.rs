use std::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:2101").await?;
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?}:{:?}", addr, String::from_utf8((&buf[..len]).to_vec()));

        let len = sock.send_to(&buf[..len], addr).await?;
    }
}
