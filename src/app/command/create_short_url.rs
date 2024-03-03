use crate::id_provider::IDProvider;

pub trait CreateShortUrlRepository {
    fn save(&self, full_url: String, id: String) -> Result<(), String>;
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

    pub async fn execute(&self, full_url: String) -> Result<String, String> {
        let id = self.id_provider.provide();

        self.repo.save(full_url, id.clone())?;

        Ok(id)
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use dashmap::DashMap;

    use crate::adapters::inmemory::InMemoryRepository;

    use super::*;

    #[tokio::test]
    async fn get_short_url() {
        // Given
        let id_provider = crate::id_provider::FakeIDProvider::new("123".to_owned());
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store);
        let command = CreateShortUrlCommand::new(id_provider, repo);

        // When
        let result = command.execute("https://www.google.com".to_owned()).await;

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
        let result = command.execute("https://www.google.com".to_owned()).await;

        let result2 = command.execute("https://www.google.com".to_owned()).await;

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
        let id = command
            .execute("https://www.google.com".to_owned())
            .await
            .unwrap();

        // Then
        assert_eq!(store.len(), 1);
        let full_url = store.get(&id).unwrap();
        assert_eq!(full_url.value(), "https://www.google.com");
    }
}
