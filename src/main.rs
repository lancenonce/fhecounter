mod client;
mod server;

#[tokio::main]
async fn main() {
    // Run the server in the background
    let server_handle = tokio::spawn(async {
        server::run_server().await;
    });

    // Run the client
    client::run_client().await;

    // Wait for the server to finish
    let _ = server_handle.await;
}
