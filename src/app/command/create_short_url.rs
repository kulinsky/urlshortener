use async_trait::async_trait;

use crate::{error::AppError, id_provider::IDProvider};

#[mockall::automock]
#[async_trait]
pub trait CreateShortUrlRepository {
    async fn save(&self, full_url: String, id: String) -> Result<(), AppError>;
}

pub struct CreateShortUrlCommand<I, R>
where
    I: IDProvider,
    R: CreateShortUrlRepository,
{
    id_provider: I,
    repo: R,
}

impl<I, R> CreateShortUrlCommand<I, R>
where
    I: IDProvider,
    R: CreateShortUrlRepository,
{
    pub fn new(id_provider: I, repo: R) -> Self {
        Self { id_provider, repo }
    }

    pub async fn execute(&self, full_url: &str) -> Result<String, AppError> {
        let parsed_url = url::Url::parse(full_url).map_err(|_| AppError::URLParseError)?;

        let id = self.id_provider.provide();

        self.repo.save(parsed_url.to_string(), id.clone()).await?;

        Ok(id)
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use dashmap::DashMap;

    use crate::{adapters::inmemory::InMemoryRepository, id_provider::MockIDProvider};

    use super::*;

    #[tokio::test]
    async fn get_short_url_with_mock() {
        // Given
        let mut stub_id_provider = MockIDProvider::new();
        stub_id_provider
            .expect_provide()
            .returning(|| "123".to_owned())
            .times(1);

        let mut mock_repo = MockCreateShortUrlRepository::new();
        mock_repo.expect_save().returning(|_, _| Ok(())).times(1);

        let sut = CreateShortUrlCommand::new(stub_id_provider, mock_repo);

        // When
        let result = sut.execute("https://www.google.com").await;

        // Then
        assert_eq!(result, Ok("123".to_owned()));
    }

    #[tokio::test]
    async fn get_short_url() {
        // Given
        let id_provider = crate::id_provider::FakeIDProvider::new("123".to_owned());
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store);
        let command = CreateShortUrlCommand::new(id_provider, repo);

        // When
        let result = command.execute("https://www.google.com").await;

        // Then
        assert_ne!(result, Ok("".to_owned()));
    }

    #[tokio::test]
    async fn get_two_different_short_url() {
        // Given
        let idp = crate::id_provider::NanoIDProvider;
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store);
        let command = CreateShortUrlCommand::new(idp, repo);

        // When
        let result = command.execute("https://www.google.com").await;

        let result2 = command.execute("https://www.google.com").await;

        // Then
        assert_ne!(result, result2);
    }

    #[tokio::test]
    async fn after_save_store_should_have_one_item() {
        // Given
        let idp = crate::id_provider::NanoIDProvider;
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store.clone());
        let command = CreateShortUrlCommand::new(idp, repo);

        // When
        let id = command.execute("https://www.google.com").await.unwrap();

        // Then
        assert_eq!(store.len(), 1);
        let full_url = store.get(&id).unwrap();
        assert_eq!(full_url.value(), "https://www.google.com/");
    }

    #[tokio::test]
    async fn test_for_invalid_url() {
        // Given
        let idp = crate::id_provider::NanoIDProvider;
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store);
        let command = CreateShortUrlCommand::new(idp, repo);

        // When
        let result = command.execute("google").await;

        // Then
        assert!(result.is_err());
        assert_eq!(result, Err(AppError::URLParseError));
    }
}
