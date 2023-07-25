use base::model::ddd::domain_service::DomainService;

use crate::aggregate::{role::model::role::Role, user::model::user::User};

#[async_trait::async_trait]
pub trait IUserDomainService: DomainService {
    async fn print_tag(&self,user: User, roles: Vec<Role>);
}
