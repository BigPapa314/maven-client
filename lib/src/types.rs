//! A library for querying maven repositories.

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub model_version: Option<String>,
    pub group_id: Option<String>,
    pub artifact_id: Option<String>,
    pub version: Option<String>,
    pub packaging: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub licenses: Option<Licenses>,
    pub developers: Option<Developers>,
    pub scm: Option<Scm>,
    pub dependencies: Option<Dependencies>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Licenses {
    #[serde(rename = "license")]
    pub values: Vec<License>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Developers {
    #[serde(rename = "developer")]
    pub values: Vec<Developer>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Developer {
    pub id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Scm {
    pub connection: Option<String>,
    pub developer_connection: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dependencies {
    #[serde(rename = "dependency")]
    pub values: Vec<Dependency>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub group_id: Option<String>,
    pub artifact_id: Option<String>,
    pub version: Option<String>,
    pub scope: Option<String>,
    //    #[serde(rename = "exclusion")]
    //pub exclusions: Vec<Exclusion>,
    pub exclusions: Option<Exclusions>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Exclusions {
    #[serde(rename = "exclusion")]
    pub values: Vec<Exclusion>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Exclusion {
    pub group_id: Option<String>,
    pub artifact_id: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub group_id: Option<String>,
    pub artifact_id: Option<String>,
    pub versioning: Versioning,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Versioning {
    pub latest: Option<String>,
    pub release: Option<String>,
    pub versions: Versions,
    pub last_updated: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Versions {
    #[serde(rename = "version")]
    pub values: Vec<String>,
}
