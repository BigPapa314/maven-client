//! A command line interface for the maven-client library.

use maven_client::test;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test().await?;
    Ok(())
}
