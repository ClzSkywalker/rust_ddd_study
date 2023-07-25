use base::model::ddd::{entity::Entity, marker_interface::MarkerInterface};

#[derive(Debug,Clone)]
pub struct Role {
    pub id: i32,
}

impl MarkerInterface for Role {}

impl Entity for Role {}
