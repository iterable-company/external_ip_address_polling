use std::sync::Arc;

use chrono::prelude::*;

use crate::{
    domain::{ExternalIPAddressRepostiory, PollingRecordRepository},
    error::Error,
    infra::{CmanExternalIPAddressRepostiory, DuckDbPollingRecordRepository},
};

pub trait RetrieveStoreExternalIPAddress<IP, Record>
where
    IP: ExternalIPAddressRepostiory,
    Record: PollingRecordRepository,
{
    async fn execute_inner(
        &self,
        external_ip_address_repository: &Arc<IP>,
        polling_record_repostiory: &Arc<Record>,
    ) -> anyhow::Result<(), Error> {
        let ip_address = external_ip_address_repository.fetch().await?;
        let now: DateTime<Utc> = Utc::now();
        polling_record_repostiory.add(ip_address, now).await?;
        Ok(())
    }
}

pub struct ConcreteRetrieveStoreExternalIPAddress {
    pub external_ip_address_repository: Arc<CmanExternalIPAddressRepostiory>,
    pub polling_record_repostiory: Arc<DuckDbPollingRecordRepository>,
}

impl<IP, Record> RetrieveStoreExternalIPAddress<IP, Record>
    for ConcreteRetrieveStoreExternalIPAddress
where
    IP: ExternalIPAddressRepostiory,
    Record: PollingRecordRepository,
{
}

impl ConcreteRetrieveStoreExternalIPAddress {
    pub async fn execute(&self) -> anyhow::Result<(), Error> {
        self.execute_inner(
            &self.external_ip_address_repository,
            &self.polling_record_repostiory,
        )
        .await
    }
}
