use base::model::ddd::{domain_service::DomainService, marker_interface::MarkerInterface};

use crate::aggregate::{
    role::model::role::Role,
    user::{iservice::iuser_domain_service::IUserDomainService, model::user::User},
};

pub struct UserDomainService {}

impl MarkerInterface for UserDomainService {}

impl DomainService for UserDomainService {}

#[async_trait::async_trait]
impl IUserDomainService for UserDomainService {
    async fn print_tag(&self,user: User, roles: Vec<Role>) {
        roles
            .iter()
            .for_each(|item| println!("name:{},role:{}", user.user_name.clone(), item.name));
    }
}
