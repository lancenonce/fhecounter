use tokio::net::TcpListener;
use tokio::io::{self, AsyncReadExt};

pub async fn run_server() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        let mut buffer = [0; 1024];
        let n = socket.read(&mut buffer).await.unwrap();

        increment_encrypted_data(&buffer[..n]);
    }
}

fn increment_encrypted_data(data: &[u8]) {
    // Here we perform FHE increment
    println!("Received data: {:?}", data);
}
