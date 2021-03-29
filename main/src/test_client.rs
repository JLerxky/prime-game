use std::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    // 连接
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    let remote_addr = "192.168.101.198:2101";
    sock.connect(remote_addr).await?;
    // 发送
    let mut buf = [255; 1024];
    let data: u128 = 99999999999999999999999999999999999999;
    let _ = sock.send(&data.to_ne_bytes()).await?;
    // 接收
    let len = sock.recv(&mut buf).await?;
    println!("{:?}:{:?}", remote_addr, &buf[..len]);
    Ok(())
}
