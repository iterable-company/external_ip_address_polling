use std::{env, sync::Arc};

use infra::{CmanExternalIPAddressRepostiory, DuckDbPollingRecordRepository};
use usecase::ConcreteRetrieveStoreExternalIPAddress;

mod domain;
mod error;
mod infra;
mod usecase;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let duckdb_file_path = &args[1];

    env_logger::init();
    let usecase = construct_usecase(duckdb_file_path);
    usecase.execute().await.unwrap();
}

fn construct_usecase(duckdb_file_path: &str) -> ConcreteRetrieveStoreExternalIPAddress {
    // since this system has only one usecase and is not intended to run in parallel, repositories don't need to be wrapped in Arc for now.
    // however, for future enhancements and to explore how to construct shared repostiories, Arc is introduced in the implementation.
    let cman = Arc::new(CmanExternalIPAddressRepostiory::new());
    let duckdb: Arc<DuckDbPollingRecordRepository> = Arc::new(DuckDbPollingRecordRepository {
        file_path: duckdb_file_path.to_string(),
    });

    ConcreteRetrieveStoreExternalIPAddress {
        external_ip_address_repository: Arc::clone(&cman),
        polling_record_repostiory: Arc::clone(&duckdb),
    }
}
