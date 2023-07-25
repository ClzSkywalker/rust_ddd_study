use base::model::ddd::{entity::Entity, marker_interface::MarkerInterface};

#[derive(Debug,Clone)]
pub struct Unit {
    pub id: i32,
}

impl MarkerInterface for Unit {
    
}

impl Entity for Unit {}
