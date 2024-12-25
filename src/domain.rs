// use std::net::Ipv4Addr directly rather than wrapping it as a domain model.
use std::net::Ipv4Addr;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::error::Error;

#[async_trait]
pub trait ExternalIPAddressRepostiory {
    async fn fetch(&self) -> Result<Ipv4Addr, Error>;
}

#[async_trait]
pub trait PollingRecordRepository {
    async fn add(&self, ip_address: Ipv4Addr, now: DateTime<Utc>) -> Result<(), Error>;
}
