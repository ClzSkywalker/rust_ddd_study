use anyhow::Result;
use base::model::ddd::{marker_interface::MarkerInterface, repository::Repository};
use domain::aggregate::user::{model::user::User, repository::user_repository::IUserRepository};

pub struct UserRepositoryImpl {}

impl MarkerInterface for UserRepositoryImpl {}

impl Repository<User, String> for UserRepositoryImpl {
    fn delete(id: String) {
        todo!()
    }

    fn by_id(id: String) -> User {
        todo!()
    }

    fn save(s: User) -> User {
        todo!()
    }

    fn save_and_flush(s: User) -> User {
        todo!()
    }
}

#[async_trait::async_trait]
impl IUserRepository for UserRepositoryImpl {
    async fn by_user_name(&self, user_name: String) -> Result<User> {
        todo!()
    }

    async fn save(&self, u: User) -> Result<User> {
        todo!()
    }
}
