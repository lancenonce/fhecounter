use std::io::Cursor;

use bincode;
use tfhe::{FheUint8, ServerKey, set_server_key};
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

pub async fn start_server(port: u16) {
    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();

    let (mut socket, _) = listener.accept().await.unwrap();
    let mut buffer = vec![0; 1024];
    socket.read_to_end(&mut buffer).await.unwrap();

    let mut serialized_data = Cursor::new(buffer);
    let server_key: ServerKey = bincode::deserialize_from(&mut serialized_data).unwrap();
    let _encrypted_initial_value: FheUint8 = bincode::deserialize_from(&mut serialized_data).unwrap();

    set_server_key(server_key);

    loop {
        // TODO: Add code to handle "increment" and "get" actions from the client
        println!("server key and initial value ready");
        break;
    }
}

