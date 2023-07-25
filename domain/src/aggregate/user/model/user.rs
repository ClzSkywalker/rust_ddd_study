use anyhow::{anyhow, Ok};
use base::model::ddd::{aggregate_root::AggregateRoot, marker_interface::MarkerInterface};
use chrono::{DateTime, Local};

use crate::share::valueobject::address::Address;

use super::{role::Role, unit::Unit};

#[derive(Debug,Clone)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub real_name: String,
    pub phone: String,
    pub password: String,
    pub address: Address,
    // 用户单位
    pub unit: Unit,
    pub roles: Vec<Role>,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

impl MarkerInterface for User {}

impl AggregateRoot for User {}

impl User {
    pub fn bind_username(&mut self, user_name: String, exist_user: User) -> anyhow::Result<()> {
        if exist_user.id != 0 {
            return Err(anyhow!("user.user.name.is.exist"));
        }
        self.user_name = user_name;
        return Ok(());
    }

    pub fn bind_role(roles: &Vec<i32>) -> Vec<Role> {
        roles
            .into_iter()
            .map(|item| Role { id: item.clone() })
            .collect()
    }
    pub fn bind_address(province: String, city: String, county: String) -> Address {
        Address {
            province: province,
            city: city,
            county: county,
        }
    }

    pub fn bind_unit(id: i32) -> Unit {
        Unit { id: id }
    }

    pub fn print_create(&self) {
        println!("新增user:{}",self.user_name);
    }
}
