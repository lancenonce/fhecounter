use std::{io};
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
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
                println!("How much would you like to increment by? Please enter a number <5");
                let mut incr = String::new();
                io::stdin().read_line(&mut incr).unwrap();
                let incr = incr.trim().parse::<u8>().expect("Please enter a value under 5");
                println!("incrementing value by {}...", incr);
                // encrypt incr, serialize it, then send it to the server to be added to the running value
                let incr = FheUint8::encrypt(incr, &client_key);
                let mut serialized_incr = Vec::new();
                bincode::serialize_into(&mut serialized_incr, &incr).unwrap();
                // write data to stream
                stream.write(&serialized_incr).await.unwrap();
            },
            "get" => {
                println!("getting value...");

                let mut buffer = vec![0; 1024];
                let bytes_read = stream.read(&mut buffer).await.unwrap();
                buffer.truncate(bytes_read);  // Truncate buffer to actual data received
            
                // Deserialize the bytes into an FheUint8 object
                let result_encrypted: FheUint8 = bincode::deserialize(&buffer).unwrap();
                // Decrypt the FheUint8 object with the client key
                let result: u8 = FheUint8::decrypt(&result_encrypted, &client_key);
            
                println!("Result: {:?}", result);
            },            
            _ => println!("Invalid command"),
        }
    }
}

