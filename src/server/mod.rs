use warp::Filter;
use bytes::Bytes;
use tfhe::{FheUint8, decrypt, ConfigBuilder};
use tfhe::prelude::*;
use crate::fhe::increment_value;

pub async fn start_server(port: u16) {
    let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buffer = vec![0; 1024];
        socket.read_to_end(&mut buffer).await.unwrap();

        // Deserialize the received data
        let (server_key, encrypted_value): (Vec<u8>, FheUint8) = deserialize(&buffer).unwrap();
        set_server_key(server_key);

        // TODO: Add code to handle "increment" and "get" actions from the client
    }
}

