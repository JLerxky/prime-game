use std::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("192.168.101.198:2101").await?;
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?}:{:?}", addr, &buf[..len]);

        let len = sock.send_to(&buf[..len], addr).await?;
    }
}
