use async_trait::async_trait;
use chrono::prelude::*;
use duckdb::{params, Connection};
use log::error;
use reqwest::Client;
use scraper::{Html, Selector};
use std::net::Ipv4Addr;

use crate::{
    domain::{ExternalIPAddressRepostiory, PollingRecordRepository},
    error::Error,
};

pub struct CmanExternalIPAddressRepostiory {
    client: Client,
}
const PAGE_URL: &str = "https://www.cman.jp/network/support/go_access.cgi";

#[async_trait]
impl ExternalIPAddressRepostiory for CmanExternalIPAddressRepostiory {
    async fn fetch(&self) -> Result<Ipv4Addr, Error> {
        let html_text = self
            .client
            .get(PAGE_URL)
            .send()
            .await
            .map_err(|e| {
                error!("failed to fetch from cman. e: {}", e);
                Error::ExternalServiceUnavailabe
            })?
            .text()
            .await
            .map_err(|e| {
                error!("failed to extract text from response. e: {}", e);
                Error::ExternalServiceSpecChanged
            })?;
        CmanExternalIPAddressRepostiory::parse(html_text)
    }
}

impl CmanExternalIPAddressRepostiory {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(), // BUG if panic
        }
    }
}

impl CmanExternalIPAddressRepostiory {
    // the structure of cman page is not part of domain knowledge.
    // it's just a cman specific knowledge. the responsibility of this repository is solely to fetch the external ip address.
    // datails of cman page structure should remain hidden from external modules even from the domain.
    fn parse(html_text: String) -> Result<Ipv4Addr, Error> {
        let html = Html::parse_document(html_text.as_str());
        let selector =
            Selector::parse("div.outIp").expect("failed to construct selector. possibly BUG");

        for element in html.select(&selector) {
            return Ok(element.inner_html().parse().map_err(|e| {
                error!(
                    "failed to convert {} into Ipv4Addr. e: {}",
                    element.inner_html(),
                    e
                );
                Error::ExternalServiceSpecChanged
            })?);
        }

        error!("faied to extract ip address tag from html");
        Err(Error::ExternalServiceSpecChanged)
    }
}

pub struct DuckDbPollingRecordRepository {
    pub file_path: String,
    // pub conn: Arc<Connection>, cannot have internal state whose type is Connection. because Connection isn't thread-safe.
}
// static CONN: Lazy<Result<Connection, Error>> = Lazy::new(|| { // also cannot define lazy variable because Connection isn't thread-safe.
//     Connection::open(DB_FILE_PATH).map_err(|e| {
//         log::error!("failed to open duckdb file. e: {}", e);
//         Error::LocalSettingInvalid
//     })
// });

#[async_trait]
impl PollingRecordRepository for DuckDbPollingRecordRepository {
    async fn add(&self, ip_address: Ipv4Addr, now: DateTime<Utc>) -> Result<(), Error> {
        let conn = Connection::open(self.file_path.as_str()).map_err(|e| {
            log::error!("failed to open duckdb file. e: {}", e);
            Error::LocalSettingInvalid
        })?;
        conn.execute(
            "INSERT INTO external_ip_address (ip_address, timestamp) VALUES (?, ?)",
            params![ip_address.to_bits(), now.timestamp()],
        )
        .map_err(|e| {
            error!("failed to insert external ip address. e: {}", e);
            Error::ExternalServiceUnavailabe
        })?;
        Ok(())
    }
}
