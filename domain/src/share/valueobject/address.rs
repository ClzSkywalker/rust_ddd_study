use base::model::ddd::{
    marker_interface::MarkerInterface,
    value_object::{self},
};

#[derive(Debug, PartialEq)]
pub struct Address {
    pub province: String,
    pub city: String,
    pub county: String,
}

impl MarkerInterface for Address {
}

impl value_object::ValueObject<Address> for Address {
    fn same_value_as(&self, other: &Address) -> bool {
        return self.eq(other);
    }
}
