use crate::{
    app::{
        command::create_short_url::{CreateShortUrlCommand, CreateShortUrlRepository},
        query::get_full_url::{GetFullUrlQuery, GetFullUrlRepository},
    },
    id_provider::IDProvider,
};

pub struct Container<I, R, Q>
where
    I: IDProvider,
    R: CreateShortUrlRepository,
    Q: GetFullUrlRepository,
{
    pub shorten_command: CreateShortUrlCommand<I, R>,
    pub get_full_url_query: GetFullUrlQuery<Q>,
}

impl<I, R, Q> Container<I, R, Q>
where
    I: IDProvider,
    R: CreateShortUrlRepository,
    Q: GetFullUrlRepository,
{
    pub fn new(id_provider: I, repository: R, querier: Q) -> Self {
        let shorten_command = CreateShortUrlCommand::new(id_provider, repository);
        let get_full_url_query = GetFullUrlQuery::new(querier);

        Container {
            shorten_command,
            get_full_url_query,
        }
    }
}

