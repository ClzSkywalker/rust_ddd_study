use base::model::ddd::{aggregate_root::AggregateRoot, marker_interface::MarkerInterface};
use chrono::{DateTime, Local};
#[derive(Debug, Clone)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl MarkerInterface for Role {}

impl AggregateRoot for Role {}

impl Role {
    pub fn is_admin(&self) -> bool {
        self.code.eq("admin")
    }
}
