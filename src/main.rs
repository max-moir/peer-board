mod core;
mod app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run the server
    app::run().await
}