//! A library for querying maven repositories.

use std::collections::HashMap;

pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn do_test() {
        test().await.expect("Something failed");
    }
}
