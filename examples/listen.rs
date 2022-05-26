//! Listens on network for nothing in particular

use kurz::Kurz;

#[tokio::main]
async fn main() {
    println!("Starting peer..");
    let kurz: Kurz<(), ()> = Kurz::new(b"11111111111111111111111111111111")
        .await
        .unwrap();

    println!("Listening..");
    kurz.listen().await;
    loop{}
}
