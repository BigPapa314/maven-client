//! A library for querying maven repositories.

use std::collections::HashMap;
use semver_rs::Version;

pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

fn test2() -> Result<(), semver_rs::Error> {
    let v1 = Version::new("1.0.0").parse()?;
    let v2 = Version::new("2.0.1-SNAPSHOT").parse()?;

    assert!(v1 < v2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn do_test() {
        test().await.expect("Something failed");
    }
    #[tokio::test]
    async fn do_test2() {
        test2().expect("Something failed");
    }
}
