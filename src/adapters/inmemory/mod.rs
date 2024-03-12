use std::sync::Arc;

use dashmap::DashMap;

use crate::error::AppError;

#[derive(Clone)]
pub struct InMemoryRepository {
    store: Arc<DashMap<String, String>>,
}

impl InMemoryRepository {
    pub fn new(store: Arc<DashMap<String, String>>) -> Self {
        Self { store }
    }
}

impl crate::app::command::create_short_url::CreateShortUrlRepository for InMemoryRepository {
    async fn save(&self, full_url: String, id: String) -> Result<(), AppError> {
        self.store.insert(id, full_url);

        Ok(())
    }
}

impl crate::app::query::get_full_url::GetFullUrlRepository for InMemoryRepository {
    async fn get(&self, id: &str) -> Result<String, AppError> {
        match self.store.get(id) {
            Some(url) => Ok(url.clone()),
            None => Err(AppError::NotFound),
        }
    }
}
