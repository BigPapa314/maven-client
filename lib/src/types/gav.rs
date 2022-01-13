use url::{ParseError, Url};

#[derive(Default, Clone, Debug)]
pub struct Gav {
    pub group_id: String,
    pub artifact_id: String,
    pub version: Option<String>,
}

impl Gav {
    pub fn new(group_id: String, artifact_id: String, version: Option<String>) -> Self {
        Gav {
            group_id,
            artifact_id,
            version,
        }
    }

    pub fn add_base_to(&self, base: Url) -> Result<Url, ParseError> {
        base.join(format!("{}/{}/", self.group_id.replace(".", "/"), self.artifact_id).as_str())
    }

    pub fn add_to(&self, base: Url) -> Result<Url, ParseError> {
        let result = self.add_base_to(base)?;
        if let Some(version) = self.version.as_ref() {
            result.join(format!("{}/", version).as_str())
        } else {
            Ok(result)
        }
    }
}
