use super::{aggregate_root::AggregateRoot, marker_interface::MarkerInterface};

pub trait Repository<AG, ID>: MarkerInterface
where
    AG: AggregateRoot,
{
    fn delete(id: ID);
    fn by_id(id: ID) -> AG;
    fn save(s: AG) -> AG;
    fn save_and_flush(s: AG) -> AG;
}
