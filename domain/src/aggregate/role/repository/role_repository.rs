use base::model::ddd::repository::Repository;

use crate::aggregate::role::model::role::Role;

#[async_trait::async_trait]
pub trait IRoleRepository: Repository<Role, String> {
    async fn list_by_ids(&self, ids: Vec<i32>) -> Vec<Role>;
}
