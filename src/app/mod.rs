pub mod cli;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    cli::run().await
}