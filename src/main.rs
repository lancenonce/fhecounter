mod client;
mod server;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    
    match mode.as_str() {
        "client" => {
            let port: u16 = args[2].parse().expect("Please provide the port number as the third argument.");
            let initial_value: u8 = args[3].parse().expect("Please provide a number less than 100 as the fourth argument.");
            client::start_client(initial_value, port).await;
        },
        "server" => {
            let port: u16 = args[2].parse().expect("Please provide the port number as the third argument.");
            server::start_server(port).await;
        },
        _ => println!("Unknown mode. Please specify 'client' or 'server' as the first argument."),
    }
}
