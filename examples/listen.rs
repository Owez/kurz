//! Listens on network for nothing in particular

use kurz::Kurz;

#[tokio::main]
async fn main() {
    let mut kurz = Kurz::new(b"11111111111111111111111111111111")
        .await
        .unwrap();
    kurz.listen().await.unwrap();
}
