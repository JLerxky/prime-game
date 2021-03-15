use tokio::io;
use tokio::net::UdpSocket;
use tokio_stream::StreamExt;
use tokio_util::codec::BytesCodec;
use tokio_util::udp::UdpFramed;

use bytes::Bytes;
use futures::SinkExt;
use std::error::Error;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

pub async fn start_client() -> Result<(), Box<dyn Error>> {
    let game_client_socket = UdpSocket::bind("127.0.0.1:0").await?;

    game_client_socket.connect("127.0.0.1:2101").await?;

    let game_client_addr = game_client_socket.local_addr()?;

    println!("游戏客户端地址: {:?}", game_client_addr);

    let mut game_client_framed = UdpFramed::new(game_client_socket, BytesCodec::new());

    let game_client_future = start_listening(&mut game_client_framed);

    send("list".to_string()).await?;

    match tokio::try_join!(game_client_future) {
        Err(e) => println!("an error occurred; error = {:?}", e),
        Ok(_) => {}
    }

    Ok(())
}

async fn send(packet: String) -> Result<(), io::Error> {

    let recv_addr = SocketAddr::from(SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 2101));

    let send_socket = UdpSocket::bind("127.0.0.1:0").await?;
    let mut send_framed = UdpFramed::new(send_socket, BytesCodec::new());

    send_framed.send((Bytes::from(packet), recv_addr)).await?;

    Ok(())
}

async fn start_listening(socket: &mut UdpFramed<BytesCodec>) -> Result<(), io::Error> {
    while let Some(Ok((bytes, _addr))) = socket.next().await {
        println!("[b] recv: {}", String::from_utf8_lossy(&bytes));

        // socket.send((Bytes::from(&b"PONG"[..]), addr)).await?;
    }

    Ok(())
}
