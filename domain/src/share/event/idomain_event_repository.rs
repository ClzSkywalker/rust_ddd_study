use base::model::ddd::marker_interface::MarkerInterface;

use super::base_domain_event::BaseDomainEvent;

pub trait IDomainEventRespository<T>: MarkerInterface
where
    T: Send + Sync,
{
    fn load(&self, id: String) -> BaseDomainEvent<T>;
    fn load_by_domain_id(&self, id: String) -> Vec<BaseDomainEvent<T>>;
    fn save(&self, event: BaseDomainEvent<T>);
    fn update(&self, event: BaseDomainEvent<T>);
}
