use anyhow::Result;
use base::model::ddd::repository::Repository;

use crate::aggregate::user::model::user::User;

#[async_trait::async_trait]
pub trait IUserRepository: Repository<User, String> {
    async fn by_user_name(&self, user_name: String) -> Result<User>;
    async fn save(&self, u: User) -> Result<User>;
}
