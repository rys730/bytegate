use std::sync::Arc;

use crate::app::{
    common::config::ServiceConfig, repository::user::UserRepositoryTrait
};

pub struct UserUsecase {
    cfg: Arc<ServiceConfig>,
    user_repository: Arc<dyn UserRepositoryTrait>
}

impl UserUsecase{
    pub fn new(cfg: Arc<ServiceConfig>, user_repository: Arc<dyn UserRepositoryTrait>) -> Self {
        Self {
            cfg: cfg,
            user_repository: user_repository
        }
    }
}