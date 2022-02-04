use std::fs::File;
use std::io::{self, Read, Write};
use std::str;
use std::time::Duration;

use async_trait::async_trait;
use regex::Regex;
use tokio_test::io::Builder;

use epp_client::domain::check::DomainCheck;
use epp_client::login::Login;
use epp_client::response::ResultCode;
use epp_client::EppClient;

const CLTRID: &str = "cltrid:1626454866";

struct TestWriter;

impl Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        print!(
            "{}",
            str::from_utf8(buf).expect("tried to log invalid UTF-8")
        );
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()
    }
}

fn log_to_stdout() -> tracing::subscriber::DefaultGuard {
    let sub = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .with_writer(|| TestWriter)
        .finish();
    tracing::subscriber::set_default(sub)
}

fn len_bytes(bytes: &str) -> [u8; 4] {
    ((bytes.len() as u32) + 4).to_be_bytes()
}

fn xml(path: &str) -> String {
    let ws_regex = Regex::new(r"[\s]{2,}").unwrap();
    let end_regex = Regex::new(r"\?>").unwrap();

    let mut f = File::open(format!("tests/resources/{}", path)).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    if !buf.is_empty() {
        let mat = end_regex.find(buf.as_str()).unwrap();
        let start = mat.end();
        buf = format!(
            "{}\r\n{}",
            &buf[..start],
            ws_regex.replace_all(&buf[start..], "")
        );
    }

    buf
}

fn build_stream(units: &[&str]) -> Builder {
    let mut builder = Builder::new();
    for (i, path) in units.iter().enumerate() {
        let buf = xml(path);
        match i % 2 {
            0 => builder.read(&len_bytes(&buf)).read(buf.as_bytes()),
            1 => builder.write(&len_bytes(&buf)).write(buf.as_bytes()),
            _ => unreachable!(),
        };
    }

    builder
}

#[tokio::test]
async fn client() {
    let _guard = log_to_stdout();

    struct FakeConnector;

    #[async_trait]
    impl epp_client::client::Connector for FakeConnector {
        type Connection = tokio_test::io::Mock;

        async fn connect(&self, _: Duration) -> Result<Self::Connection, epp_client::Error> {
            Ok(build_stream(&[
                "response/greeting.xml",
                "request/login.xml",
                "response/login.xml",
                "request/domain/check.xml",
                "response/domain/check.xml",
            ])
            .build())
        }
    }

    let mut client = EppClient::new(FakeConnector, "test".into(), Duration::from_secs(5))
        .await
        .unwrap();

    assert_eq!(client.xml_greeting(), xml("response/greeting.xml"));
    client
        .transact(
            &Login::new(
                "username",
                "password",
                Some(&["http://schema.ispapi.net/epp/xml/keyvalue-1.0"]),
            ),
            CLTRID,
        )
        .await
        .unwrap();

    let rsp = client
        .transact(
            &DomainCheck {
                domains: &["eppdev.com", "eppdev.net"],
            },
            CLTRID,
        )
        .await
        .unwrap();
    assert_eq!(rsp.result.code, ResultCode::CommandCompletedSuccessfully);

    let result = rsp.res_data().unwrap();
    assert_eq!(result.list[0].id, "eppdev.com");
}
