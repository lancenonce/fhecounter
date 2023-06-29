use std::io::Cursor;
use bincode;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::spawn;

#[allow(unused_imports)]
use tfhe::{FheUint8, ServerKey, set_server_key};

pub async fn start_server(port: u16) {
    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        spawn(handle_client(socket)); // New task for each client
    }
}

async fn handle_client(mut socket: TcpStream) {
    let mut buffer = vec![0; 1024];
    socket.read_to_end(&mut buffer).await.unwrap();
    let mut cursor = Cursor::new(buffer);

    match (bincode::deserialize_from::<_, ServerKey>(&mut cursor), bincode::deserialize_from::<_, FheUint8>(&mut cursor)) {
        (Ok(server_key), Ok(current_value)) => {
            let mut result = current_value.clone();
            set_server_key(server_key);

            println!("server key and initial value ready");

            loop {
                let mut buffer = vec![0; 1024];
                socket.read_to_end(&mut buffer).await.unwrap();
                if buffer.is_empty() {
                    // Connection was closed
                    break;
                }

                let mut cursor = Cursor::new(buffer);
                if let Ok(increment_value) = bincode::deserialize_from::<_, FheUint8>(&mut cursor) {
                    println!("Increment value received. Crunching numbers...");
                    // Increment current_value by increment_value
                    result += increment_value;

                    // Serialize the result and write to the socket
                    let result_bytes = bincode::serialize(&result).unwrap();
                    if let Err(e) = socket.write_all(&result_bytes).await {
                        println!("Failed to write to socket: {}", e);
                    }
                }
            }
        }
        _ => {
            println!("Failed to deserialize server key and/or initial value");
        }
    }
}
