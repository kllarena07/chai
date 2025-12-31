mod app;
mod server;

use crate::{app::MyApp, server::AppServer};

#[tokio::main]
async fn main() {
    let mut server = AppServer::<MyApp>::new();
    server.run().await.expect("Failed running server");
}
