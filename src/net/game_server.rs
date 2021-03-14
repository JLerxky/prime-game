use tokio::net::UdpSocket;
use tokio::{io, time};
use tokio_stream::StreamExt;
use tokio_util::codec::BytesCodec;
use tokio_util::udp::UdpFramed;

use bytes::Bytes;
use futures::{FutureExt, SinkExt};
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::time::Duration;

// #[tokio::main]
async fn game_server_start() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:0".to_string());

    // Bind both our sockets and then figure out what ports we got.
    let a = UdpSocket::bind(&addr).await?;
    let b = UdpSocket::bind(&addr).await?;

    let b_addr = b.local_addr()?;

    let mut a = UdpFramed::new(a, BytesCodec::new());
    let mut b = UdpFramed::new(b, BytesCodec::new());

    // Start off by sending a ping from a to b, afterwards we just print out
    // what they send us and continually send pings
    let a = ping(&mut a, b_addr);

    // The second client we have will receive the pings from `a` and then send
    // back pongs.
    let b = pong(&mut b);

    // Run both futures simultaneously of `a` and `b` sending messages back and forth.
    match tokio::try_join!(a, b) {
        Err(e) => println!("an error occurred; error = {:?}", e),
        _ => println!("done!"),
    }

    Ok(())
}

async fn ping(socket: &mut UdpFramed<BytesCodec>, b_addr: SocketAddr) -> Result<(), io::Error> {
    socket.send((Bytes::from(&b"PING"[..]), b_addr)).await?;

    for _ in 0..4usize {
        let (bytes, addr) = socket.next().map(|e| e.unwrap()).await?;

        println!("[a] recv: {}", String::from_utf8_lossy(&bytes));

        socket.send((Bytes::from(&b"PING"[..]), addr)).await?;
    }

    Ok(())
}

async fn pong(socket: &mut UdpFramed<BytesCodec>) -> Result<(), io::Error> {
    let timeout = Duration::from_millis(200);

    while let Ok(Some(Ok((bytes, addr)))) = time::timeout(timeout, socket.next()).await {
        println!("[b] recv: {}", String::from_utf8_lossy(&bytes));

        socket.send((Bytes::from(&b"PONG"[..]), addr)).await?;
    }

    Ok(())
}