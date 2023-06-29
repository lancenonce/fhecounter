use std::io::Cursor;

use bincode;
use tfhe::{FheUint8, ServerKey, set_server_key};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::spawn;

pub async fn start_server(port: u16) {
    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        spawn(handle_client(socket)); // New task for each client
    }
}

async fn handle_client(mut socket: TcpStream) {
    // Read server key and initial value once at the start
    let mut buffer = vec![0; 1024];
    let bytes_read = socket.read(&mut buffer).await.unwrap();
    buffer.truncate(bytes_read);  // Truncate buffer to actual data received

    let mut serialized_data = Cursor::new(buffer);
    let server_key: ServerKey = bincode::deserialize_from(&mut serialized_data).unwrap();
    let current_value: FheUint8 = bincode::deserialize_from(&mut serialized_data).unwrap();
    let mut result = current_value.clone();

    set_server_key(server_key);

    println!("server key and initial value ready");

    loop {
        // Read increment value
        let mut buffer = vec![0; 1024];
        let bytes_read = socket.read(&mut buffer).await.unwrap();
        if bytes_read == 0 {
            // Connection was closed
            break;
        }
        buffer.truncate(bytes_read);  // Truncate buffer to actual data received

        let mut serialized_data = Cursor::new(buffer);
        let increment_value: FheUint8 = bincode::deserialize_from(&mut serialized_data).unwrap();
        println!("Increment value received. Crunching numbers...");

        // Increment current_value by increment_value
        result += increment_value;

        // Serialize the result and write to the socket
        let result_bytes = bincode::serialize(&result).unwrap();
        socket.write_all(&result_bytes).await.unwrap();
    }
}



