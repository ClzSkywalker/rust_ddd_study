use base::model::ddd::marker_interface::MarkerInterface;
use base::model::ddd::repository::Repository;
use domain::aggregate::role::model::role::Role;
use domain::aggregate::role::repository::role_repository::IRoleRepository;

pub struct RoleRepositoryImpl {}

impl MarkerInterface for RoleRepositoryImpl {}

impl Repository<Role, String> for RoleRepositoryImpl {
    fn delete(id: String) {
        todo!()
    }

    fn by_id(id: String) -> Role {
        todo!()
    }

    fn save(s: Role) -> Role {
        todo!()
    }

    fn save_and_flush(s: Role) -> Role {
        todo!()
    }
}

#[async_trait::async_trait]
impl IRoleRepository for RoleRepositoryImpl {
    async fn list_by_ids(&self, ids: Vec<i32>) -> Vec<Role> {
        todo!()
    }
}
