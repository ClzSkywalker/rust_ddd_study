use domain::aggregate::user::model::user::User;

use crate::db::model::user_po::UserPo;

pub fn deserialize(po: UserPo) -> User {
    User {
        id: po.base.id,
        user_name: po.user_name,
        real_name: po.real_name,
        phone: po.phone,
        password: po.password,
        address: User::bind_address(po.provence, po.city, po.county),
        unit: User::bind_unit(po.unit_id),
        roles: User::bind_role(&po.roleIds),
        created_at: po.base.created_at,
        updated_at: po.base.updated_at,
    }
}

pub fn serialize(u: User) -> UserPo {
    UserPo {
        base: base::model::result::base_uuid_entity::BaseUuidEntity {
            id: u.id,
            created_at: u.created_at,
            updated_at: u.updated_at,
            deleted_at: None,
        },
        user_name: u.user_name,
        real_name: u.real_name,
        phone: u.phone,
        password: u.password,
        unit_id: u.unit.id,
        roleIds: u.roles.iter().map(|item| item.id).collect(),
        provence: u.address.province,
        city: u.address.city,
        county: u.address.county,
    }
}
