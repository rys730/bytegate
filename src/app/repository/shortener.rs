pub struct ShortenerRepository{}

pub trait ShortenerRepositoryTrait: Send + Sync {
    fn create_url_mapping(&self, url: String) -> String;
}

impl ShortenerRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl ShortenerRepositoryTrait for ShortenerRepository {
    fn create_url_mapping(&self, url: String) -> String {
        url
    }
}




