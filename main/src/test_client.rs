use std::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    // 连接
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    let remote_addr = "0.0.0.0:2101";
    sock.connect(remote_addr).await?;
    // 发送
    let mut buf = [255; 1024];
    // u128
    // let data: u128 = 340282366920938463463374607431768211455;
    // let _ = sock.send(&data.to_ne_bytes()).await?;
    // 字符
    let data: &str = ",";
    let _ = sock.send(data.as_bytes()).await?;
    // 接收
    let len = sock.recv(&mut buf).await?;
    println!("{:?}:{:?}", remote_addr, &buf[..len]);
    Ok(())
}
