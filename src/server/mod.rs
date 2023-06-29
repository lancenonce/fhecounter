use std::io::Cursor;

use bincode;
use tfhe::{FheUint8, ServerKey, set_server_key};
use tokio::io::AsyncReadExt;
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
    let mut buffer = vec![0; 1024];
    socket.read_to_end(&mut buffer).await.unwrap();

    let mut serialized_data = Cursor::new(buffer);
    let server_key: ServerKey = bincode::deserialize_from(&mut serialized_data).unwrap();
    let _encrypted_initial_value: FheUint8 = bincode::deserialize_from(&mut serialized_data).unwrap();

    set_server_key(server_key);

    // TODO: Add code to handle "increment" and "get" actions from the client
    println!("server key and initial value ready");
}

