use crate::types::Gav;
use crate::types::Metadata;
use quick_xml::de::{from_str, DeError};
use reqwest::Response;
use url::Url;

pub fn make_metadata_uri(base: Url, gav: Gav) -> Result<Url, Box<dyn std::error::Error>> {
    let target = gav.add_base_to(base)?;
    println!("{}", target);
    let target = target.join("maven-metadata.xml")?;
    println!("{}", target);
    Ok(target)
}

pub async fn fetch_metadata(base: Url, gav: Gav) -> Result<Metadata, Box<dyn std::error::Error>> {
    let meta_data_uri = make_metadata_uri(base, gav)?;
    let meta_data_text = reqwest::get(meta_data_uri).await?.text().await?;
    let meta_data: Metadata =
        quick_xml::de::from_str(meta_data_text.as_str()).expect("Cannot parse meta-data.xml");
    Ok(meta_data)
}
