//! A library for querying maven repositories.

mod tools;
mod types;

use semver::Version;
use std::collections::HashMap;

pub async fn print_versions(
    group: String,
    artifact: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let base = url::Url::parse("https://repo1.maven.org/maven2/")?;
    let gav = types::Gav::new(group, artifact, None);
    let meta_data_uri = tools::fetch_metadata(base, gav).await?;
    dbg!(meta_data_uri);
    Ok(())
}

pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

fn test2() -> Result<(), semver::Error> {
    let v1 = Version::parse("1.0.0")?;
    let v2 = Version::parse("2.0.1-SNAPSHOT")?;

    assert!(v1 < v2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use quick_xml::de::{from_str, DeError};

    #[tokio::test]
    async fn test_versions() {
        print_versions("org.yakworks".to_owned(), "commons".to_owned())
            .await
            .expect("Something failed");
    }

    #[tokio::test]
    async fn do_test() {
        test().await.expect("Something failed");
    }
    #[tokio::test]
    async fn do_test2() {
        test2().expect("Something failed");
    }

    #[tokio::test]
    async fn do_test3() {
        let pom = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <project xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd" xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
          <modelVersion>4.0.0</modelVersion>
          <groupId>com.impartus</groupId>
          <artifactId>app</artifactId>
          <version>0.0.1</version>
          <packaging>aar</packaging>
          <name>Impartus Activity</name>
          <description>Impartus Activity</description>
          <url>https://www.impartus.com</url>
          <licenses>
            <license>
              <name>The Apache License, Version 2.0</name>
              <url>http://www.apache.org/licenses/LICENSE-2.0.txt</url>
            </license>
          </licenses>
          <developers>
            <developer>
              <id>Impartus</id>
              <name>Impartus</name>
              <email>apps@impartus.com</email>
            </developer>
          </developers>
          <scm>
            <connection>scm:git:git://bitbucket.org/impartusdev/impartus-libwebrtc.git</connection>
            <developerConnection>scm:git:ssh://bitbucket.org:impartusdev/impartus-libwebrtc.git</developerConnection>
            <url>https://bitbucket.org/impartusdev/impartus-libwebrtc/src</url>
          </scm>
          <dependencies>
            <dependency>
              <groupId>com.google.android.exoplayer</groupId>
              <artifactId>exoplayer</artifactId>
              <version>r1.5.2</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.google.android.material</groupId>
              <artifactId>material</artifactId>
              <version>1.2.1</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>androidx.cardview</groupId>
              <artifactId>cardview</artifactId>
              <version>1.0.0</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>androidx.recyclerview</groupId>
              <artifactId>recyclerview</artifactId>
              <version>1.1.0</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>androidx.legacy</groupId>
              <artifactId>legacy-support-v4</artifactId>
              <version>1.0.0</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>de.hdodenhof</groupId>
              <artifactId>circleimageview</artifactId>
              <version>2.2.0</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.weiwangcn.betterspinner</groupId>
              <artifactId>library-material</artifactId>
              <version>1.1.0</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.android.volley</groupId>
              <artifactId>volley</artifactId>
              <version>1.1.1</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.bignerdranch.android</groupId>
              <artifactId>expandablerecyclerview</artifactId>
              <version>1.0.3</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.google.code.gson</groupId>
              <artifactId>gson</artifactId>
              <version>2.8.5</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.davemorrissey.labs</groupId>
              <artifactId>subsampling-scale-image-view</artifactId>
              <version>3.1.3</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.github.arimorty</groupId>
              <artifactId>floatingsearchview</artifactId>
              <version>2.0.3</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>jp.wasabeef</groupId>
              <artifactId>glide-transformations</artifactId>
              <version>1.3.1</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.github.bumptech.glide</groupId>
              <artifactId>glide</artifactId>
              <version>4.11.0</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.google.firebase</groupId>
              <artifactId>firebase-bom</artifactId>
              <version>26.1.1</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.google.firebase</groupId>
              <artifactId>firebase-crashlytics</artifactId>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.google.firebase</groupId>
              <artifactId>firebase-analytics</artifactId>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.google.firebase</groupId>
              <artifactId>firebase-core</artifactId>
              <version>18.0.0</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.google.firebase</groupId>
              <artifactId>firebase-messaging</artifactId>
              <version>21.0.1</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.github.PhilJay</groupId>
              <artifactId>MPAndroidChart</artifactId>
              <version>v3.0.2</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.github.freshdesk</groupId>
              <artifactId>freshchat-android</artifactId>
              <version>1.5.3</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.prolificinteractive</groupId>
              <artifactId>material-calendarview</artifactId>
              <version>1.4.3</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>net.gotev</groupId>
              <artifactId>uploadservice</artifactId>
              <version>3.4.2</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>androidx.appcompat</groupId>
              <artifactId>appcompat</artifactId>
              <version>1.3.0-alpha02</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>androidx.constraintlayout</groupId>
              <artifactId>constraintlayout</artifactId>
              <version>2.0.4</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.github.yalantis</groupId>
              <artifactId>ucrop</artifactId>
              <version>2.2.6-native</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>org.json</groupId>
              <artifactId>json</artifactId>
              <version>20180813</version>
              <scope>compile</scope>
            </dependency>
            <dependency>
              <groupId>com.impartus.live</groupId>
              <artifactId>libwebrtc-impartus</artifactId>
              <version>1.0</version>
              <type>aar</type>
              <scope>compile</scope>
              <exclusions>
                <exclusion>
                  <artifactId>*</artifactId>
                  <groupId>*</groupId>
                </exclusion>
              </exclusions>
            </dependency>
          </dependencies>
        </project>
        "#;
        println!("{}", pom);
        let project: types::Project = from_str(pom).expect("Something failed");
        assert!(project.name.as_ref().unwrap() == "Impartus Activity");
        dbg!(project);
    }

    #[tokio::test]
    async fn do_test4() {
        let mvn_metadata = r#"
        <metadata modelVersion="1.1.0">
            <groupId>com.helmut-fischer.series-a.display-unit.firmware</groupId>
            <artifactId>application</artifactId>
            <versioning>
                <latest>2.1.0-SNAPSHOT</latest>
                <release>0.1.0</release>
                <versions>
                    <version>1.0.0+3</version>
                    <version>1.0.1+5</version>
                    <version>1.0.2+7</version>
                    <version>1.1.0+6</version>
                    <version>1.1.1+40</version>
                    <version>2.0.0+301</version>
                    <version>0.1.0</version>
                    <version>0.2.0-SNAPSHOT</version>
                    <version>1.0.2-SNAPSHOT</version>
                    <version>1.1.0-SNAPSHOT</version>
                    <version>1.2.0-SNAPSHOT</version>
                    <version>2.1.0-SNAPSHOT</version>
                </versions>
                <lastUpdated>20220113095524</lastUpdated>
            </versioning>
        </metadata>
        "#;
        println!("{}", mvn_metadata);
        let metadata: types::Metadata = from_str(mvn_metadata).expect("Something failed");
        assert!(
            metadata.group_id.as_ref().unwrap()
                == "com.helmut-fischer.series-a.display-unit.firmware"
        );
        dbg!(metadata);
    }
}
