//! Module for automated tests

use std::{error::Error, fs::File, io::Read};

use regex::Regex;
use similar_asserts::assert_eq;

use crate::{
    client::RequestData,
    common::NoExtension,
    request::{Command, CommandWrapper, Extension, Transaction},
    response::Response,
    xml,
};

pub(crate) const RESOURCES_DIR: &str = "./tests/resources";
pub(crate) const CLTRID: &str = "cltrid:1626454866";
pub(crate) const SVTRID: &str = "RO-6879-1627224678242975";
pub(crate) const SUCCESS_MSG: &str = "Command completed successfully";

/// Reads EPP XML requests and responses from the test/resources directory to run tests on
pub(crate) fn get_xml(path: &str) -> Result<String, Box<dyn Error>> {
    let ws_regex = Regex::new(r"[\s]{2,}")?;

    let mut f = File::open(format!("{RESOURCES_DIR}/{path}"))?;
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
    let document = CommandWrapper::new(req.command, req.extension, CLTRID);
    assert_eq!(expected, xml::serialize(document).unwrap());
}

pub(crate) fn response_from_file<'c, Cmd>(
    path: &str,
) -> Response<Cmd::Response, <NoExtension as Extension>::Response>
where
    Cmd: Transaction<NoExtension> + Command + 'c,
{
    response_from_file_with_ext::<Cmd, NoExtension>(path)
}

pub(crate) fn response_from_file_with_ext<Cmd, Ext>(
    path: &str,
) -> Response<Cmd::Response, Ext::Response>
where
    Cmd: Transaction<NoExtension> + Command,
    Ext: Extension,
{
    let xml = get_xml(path).unwrap();
    dbg!(&xml);
    let rsp = xml::deserialize::<Response<Cmd::Response, Ext::Response>>(&xml).unwrap();
    assert!(rsp.result.code.is_success());
    rsp
}
