use base::model::ddd::{command::Command, marker_interface::MarkerInterface};
use chrono::Local;
use domain::aggregate::user::model::user::User;

pub struct CreateUserAbilityCommand {
    pub user_name: String,
    pub real_name: String,
    pub phone: String,
    pub province: String,
    pub city: String,
    pub county: String,
    pub roles: Vec<i32>,
    pub unit_id: i32,
    pub password: String,
}

impl MarkerInterface for CreateUserAbilityCommand {}

impl Command for CreateUserAbilityCommand {}

impl CreateUserAbilityCommand {
    pub fn to_user(&self) -> User {
        User {
            id: 0,
            user_name: self.user_name.clone(),
            real_name: self.real_name.clone(),
            phone: self.phone.clone(),
            password: self.password.clone(),
            address: User::bind_address(
                self.province.clone(),
                self.city.clone(),
                self.county.clone(),
            ),
            unit: User::bind_unit(self.unit_id),
            roles: User::bind_role(&self.roles),
            created_at: Local::now(),
            updated_at: None,
        }
    }
}
