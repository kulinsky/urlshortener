pub mod command;
pub mod query;

#[cfg(test)]
mod tests {
    use dashmap::DashMap;
    use std::sync::Arc;

    use crate::adapters::inmemory::InMemoryRepository;

    #[tokio::test]
    async fn create_and_get_short_url() {
        // Given
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store.clone());

        let create_command = crate::app::command::create_short_url::CreateShortUrlCommand::new(
            crate::id_provider::NanoIDProvider,
            repo.clone(),
        );

        let get_query = crate::app::query::get_full_url::GetFullUrlQuery::new(repo);

        // When
        let result = create_command
            .execute("https://www.google.com".to_owned())
            .await;
        let result2 = get_query.execute(&result.unwrap()).await.unwrap();

        // Then
        assert_eq!(result2, "https://www.google.com".to_owned());
    }
}
