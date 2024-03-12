use std::sync::Arc;

use dashmap::DashMap;

use crate::{adapters::inmemory::InMemoryRepository, ports::httpapi::Server};

pub mod adapters;
pub mod app;
pub mod di;
pub mod error;
pub mod id_provider;
pub mod ports;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let store = Arc::new(DashMap::new());
    let repo = InMemoryRepository::new(store.clone());
    let querier = InMemoryRepository::new(store.clone());

    let idp = id_provider::NanoIDProvider;
    let container = Arc::new(di::Container::new(idp, repo, querier));

    let server = Server::new(3001, container);

    server.run().await;
}
