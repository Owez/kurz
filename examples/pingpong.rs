//! Send a debug ping-pong request and response between two peers

use kurz::{Kurz, Peer, message::Request};

#[tokio::main]
async fn main() {
    // Create peers
    println!("Creating peers..");
    let key = b"11111111111111111111111111111111";
    let mut uno = Kurz::new(key).await.unwrap();
    let mut dos = Kurz::new_custom("0.0.0.0:7668".parse().unwrap(),key).await.unwrap();

    // Listen for both
    println!("Starting listeners..");
    uno.listen().await.unwrap();
    dos.listen().await.unwrap();

    // Send request; dos will fail as listeners only expect requests and we're not handling ping-pongs properly here
    println!("Sending pingpong request");
    let uno_peer = Peer { addr: "0.0.0.0:7667".parse().unwrap()}; 
    dos.send_debug(&uno_peer, Request::PingPong).await.unwrap();
}