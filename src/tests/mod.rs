//! Module for automated tests

use std::{error::Error, fs::File, io::Read};

use regex::Regex;

use crate::{
    client::RequestData,
    request::{Command, Extension, Transaction},
    xml::EppXml,
};

pub(crate) const RESOURCES_DIR: &str = "./tests/resources";
pub(crate) const CLTRID: &str = "cltrid:1626454866";
pub(crate) const SVTRID: &str = "RO-6879-1627224678242975";
pub(crate) const SUCCESS_MSG: &str = "Command completed successfully";

/// Reads EPP XML requests and responses from the test/resources directory to run tests on
pub(crate) fn get_xml(path: &str) -> Result<String, Box<dyn Error>> {
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

pub(crate) fn assert_serialized<'c, 'e, Cmd, Ext>(
    path: &str,
    req: impl Into<RequestData<'c, 'e, Cmd, Ext>>,
) where
    Cmd: Transaction<Ext> + Command + 'c,
    Ext: Extension + 'e,
{
    let expected = get_xml(path).unwrap();
    let req = req.into();
    let document = <Cmd as Transaction<Ext>>::command(req.command, req.extension, CLTRID);
    let actual = EppXml::serialize(&document).unwrap();
    assert_eq!(expected, actual);
}
