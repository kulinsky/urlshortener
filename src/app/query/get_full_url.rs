use crate::error::AppError;

pub trait GetFullUrlRepository {
    fn get(
        &self,
        id: &str,
    ) -> impl std::future::Future<Output = Result<String, AppError>> + std::marker::Send;
}

pub struct GetFullUrlQuery<R>
where
    R: GetFullUrlRepository,
{
    repo: R,
}

impl<R> GetFullUrlQuery<R>
where
    R: GetFullUrlRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: &str) -> Result<String, AppError> {
        self.repo.get(id).await
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use dashmap::DashMap;

    use crate::adapters::inmemory::InMemoryRepository;

    use super::*;

    #[tokio::test]
    async fn get_full_url() {
        // Given
        struct FakeRepository;
        impl GetFullUrlRepository for FakeRepository {
            async fn get(&self, _id: &str) -> Result<String, AppError> {
                Ok("https://www.google.com".to_owned())
            }
        }

        let repo = FakeRepository;
        let query = GetFullUrlQuery::new(repo);

        // When
        let result = query.execute("123").await;

        // Then
        assert_eq!(result, Ok("https://www.google.com".to_owned()));
    }

    #[tokio::test]
    async fn get_from_inmemory_repo() {
        // Given
        let store = Arc::new(DashMap::new());
        store.insert("123".to_owned(), "https://www.google.com".to_owned());
        let repo = InMemoryRepository::new(store);
        let query = GetFullUrlQuery::new(repo);

        // When
        let result = query.execute("123").await;

        // Then
        assert_eq!(result, Ok("https://www.google.com".to_owned()));
    }

    #[tokio::test]
    async fn get_two_different_full_url() {
        // Given
        let store = Arc::new(DashMap::new());
        store.insert("123".to_owned(), "https://www.google.com".to_owned());
        store.insert("456".to_owned(), "https://www.github.com".to_owned());
        let repo = InMemoryRepository::new(store);
        let query = GetFullUrlQuery::new(repo);

        // When
        let result1 = query.execute("123").await;
        let result2 = query.execute("456").await;

        // Then
        assert_eq!(result1, Ok("https://www.google.com".to_owned()));
        assert_eq!(result2, Ok("https://www.github.com".to_owned()));
    }
}
