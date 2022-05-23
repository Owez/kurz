//! Send a debug ping-pong request and response between two peers

use kurz::{message::Request, Kurz, Peer};

#[tokio::main]
async fn main() {
    // Create peers
    println!("Creating peers..");
    let key = b"11111111111111111111111111111111";
    let uno = Kurz::new(key).await.unwrap();
    let dos = Kurz::new_custom("0.0.0.0:7668".parse().unwrap(), key)
        .await
        .unwrap();

    // Listen for both
    println!("Starting listeners..");
    uno.listen().await;
    dos.listen().await;

    // Send requests continuously
    let uno_peer = Peer {
        addr: "0.0.0.0:7667".parse().unwrap(),
    };
    let mut count: u128 = 0;
    loop {
        count += 1;
        if count % 1000 == 0 {
            println!("Sending ping-pong request #{}", count);
        }
        dos.send_debug(&uno_peer, Request::PingPong).await.unwrap();
    }
}
