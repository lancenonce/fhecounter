use std::{io};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tfhe::{ConfigBuilder, generate_keys, FheUint8};
use tfhe::prelude::*;
use bincode;

pub async fn start_client(initial_value: u8, port: u16) {
    // Generate keys on the client for privacy
    let config = ConfigBuilder::all_disabled().enable_default_uint8().build();
    let (client_key, server_key) = generate_keys(config);

    // Encrypt the initial value
    let current_val = FheUint8::encrypt(initial_value, &client_key);

    // Serialize the initial value and the server key to send to the server
    let mut serialized_data = Vec::new();
    bincode::serialize_into(&mut serialized_data, &server_key).unwrap();
    bincode::serialize_into(&mut serialized_data, &current_val).unwrap();

    // Connect to the server
    let mut stream = TcpStream::connect(("127.0.0.1", port)).await.unwrap();

    // Send the data
    stream.write_all(&serialized_data).await.unwrap();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        match input {
            "increment" => {
                // TODO: Add code to encrypt 1, send to the server, and update current_val
                println!("incrementing value...");
            },
            "get" => {
                // TODO: Add code to retrieve the current value from the server and decrypt it
                println!("getting value...");
            },
            _ => println!("Invalid command"),
        }
    }
}

