use std::fmt::Display;

use regex::Regex;
use color_eyre::{Result, eyre::eyre};


pub struct S3Location{
    pub bucket: String,
    pub prefix: String,
}
impl S3Location{
    pub fn parse(s3_location: &str) -> Result<S3Location>{
        let s3_path_re = Regex::new(
                // https://regex101.com/r/wAmOQU/1
                r#"^([Ss]3://)?(?P<bucket>[^/]*)(?P<prefix>[\w/.-]*)$"#,
            )?;

            let captures = s3_path_re
                .captures(s3_location)
                .ok_or_else(|| eyre!("No regex matches"))?;
            let bucket = captures.name("bucket").unwrap().as_str().to_string();
            let prefix = captures
                .name("prefix")
                .unwrap()
                .as_str();
            let prefix = prefix.strip_prefix('/').unwrap_or(prefix);
            let prefix = prefix.strip_suffix('/').unwrap_or(prefix).to_string();

        Ok(S3Location{ bucket, prefix })
    }
}
impl Display for S3Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("s3://{}/{}", self.bucket, self.prefix))
    }
}