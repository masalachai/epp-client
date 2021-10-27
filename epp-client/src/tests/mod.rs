//! Module for automated tests

pub mod de;
pub mod se;

use regex::Regex;
use std::{error::Error, fs::File, io::Read};

const RESOURCES_DIR: &str = "./test/resources";
const CLTRID: &str = "cltrid:1626454866";

/// Reads EPP XML requests and responses from the test/resources directory to run tests on
fn get_xml(path: &str) -> Result<String, Box<dyn Error>> {
    let ws_regex = Regex::new(r"[\s]{2,}")?;

    let mut f = File::open(format!("{}/{}", RESOURCES_DIR, path))?;
    let mut buf = String::new();

    f.read_to_string(&mut buf)?;
    if !buf.is_empty() {
        let mat = Regex::new(r"\?>").unwrap().find(buf.as_str()).unwrap();
        let start = mat.end();
        buf = format!(
            "{}\r\n{}",
            &buf[..start],
            ws_regex.replace_all(&buf[start..], "")
        );
    }
    Ok(buf)
}
