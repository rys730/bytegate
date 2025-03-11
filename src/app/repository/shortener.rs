use crate::app::infrastructure::postgres::postgres::DB;

pub struct ShortenerRepository{
    db: DB
}

pub trait ShortenerRepositoryTrait: Send + Sync {
    fn create_url_mapping(&self, url: String) -> String;
}

impl ShortenerRepository {
    pub fn new(db: DB) -> Self {
        Self {db: db}
    }
}

impl ShortenerRepositoryTrait for ShortenerRepository {
    fn create_url_mapping(&self, url: String) -> String {
        url
    }
}




