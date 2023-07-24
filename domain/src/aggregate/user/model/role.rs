use base::model::ddd::{entity::Entity, marker_interface::MarkerInterface};

pub struct Role {
    pub id: i32,
}

impl MarkerInterface for Role {}

impl Entity for Role {}
