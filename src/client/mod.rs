use std::env;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tfhe::{ConfigBuilder, generate_keys, FheUint8};
use tfhe::prelude::*;
use bincode::serialize;

pub async fn start_client(port: u16) {
    // Get the initial number from the command line
    let args: Vec<String> = env::args().collect();
    let initial_value: u8 = args[2].parse().expect("Please provide a number less than 100 as the third argument.");

    // Generate keys
    let config = ConfigBuilder::all_disabled().enable_default_uint8().build();
    let (client_key, server_key) = generate_keys(config);

    // Encrypt the initial value
    let initial_value = FheUint8::encrypt(initial_value, &client_key);

    // Serialize the initial data to send to the server
    let data = serialize(&(server_key, initial_value)).unwrap();

    // Connect to the server
    let mut stream = TcpStream::connect(("127.0.0.1", port)).await.unwrap();

    // Send the data
    stream.write_all(&data).await.unwrap();

    // TODO: Add code to handle "increment" and "receive number" actions
}
