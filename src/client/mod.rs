use std::env;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tfhe::{ConfigBuilder, generate_keys, FheUint8};
use tfhe::prelude::*;
use bincode::serialize;

pub async fn start_client(initial_value: u8, port: u16) {
    // Generate keys on the client for privacy
    let config = ConfigBuilder::all_disabled().enable_default_uint8().build();
    let (client_key, server_key) = generate_keys(config);

    // Encrypt the initial value
    let mut current_val = FheUint8::encrypt(initial_value, &client_key);

    // Serialize the initial data to send to the server
    let data = serialize(&(server_key, current_val)).unwrap();

    // Connect to the server
    let mut stream = TcpStream::connect(("127.0.0.1", port)).await.unwrap();

    // Send the data
    stream.write_all(&data).await.unwrap();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        match input {
            "increment" => {
                // TODO: Add code to encrypt 1, send to the server, and update current_val
            },
            "get" => {
                // TODO: Add code to retrieve the current value from the server and decrypt it
            },
            _ => println!("Invalid command"),
        }
    }
}

