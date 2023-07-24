use super::marker_interface::MarkerInterface;

pub trait ValueObject<T>: MarkerInterface {
    fn same_value_as(&self, other: &T) -> bool;
}
