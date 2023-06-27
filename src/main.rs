mod client;
mod server;

#[tokio::main]
async fn main() {
    let port = 8080;
    let server_future = server::start_server(port);
    let client_future = client::start_client(port);
    futures::join!(server_future, client_future);
}
