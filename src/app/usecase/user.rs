use std::sync::Arc;

use crate::app::{common::{errors::Result, utils}, model::dto::user::{RegisterUserRequest, RegisterUserResponse, UserDB}, repository::user::{UserRepository, UserRepositoryTrait}};

pub struct UserUsecase {
    user_repository: Arc<dyn UserRepositoryTrait>
}

impl UserUsecase {
    pub fn new(repo: Arc<UserRepository>) -> Self {
        Self {
            user_repository: repo
        }
    }

    pub async fn register_user (&self, request: RegisterUserRequest) -> Result<RegisterUserResponse> {
        let session = utils::generate_user_session().to_string();
        let res = self.user_repository.create_user(UserDB{
            username: request.username,
            password: request.password,
            session: session,
            ..Default::default()
        }).await?;
        Ok(RegisterUserResponse { session: res.session })
    }
}