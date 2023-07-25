use base::model::ddd::marker_interface::MarkerInterface;
use domain::share::event::idomain_event_repository::IDomainEventRespository;

pub struct DomainEventRepository {}
impl MarkerInterface for DomainEventRepository {}

impl<T> IDomainEventRespository<T> for DomainEventRepository
where
    T: Send + Sync,
{
    fn load(&self, id: String) -> domain::share::event::base_domain_event::BaseDomainEvent<T> {
        todo!()
    }

    fn load_by_domain_id(
        &self,
        id: String,
    ) -> Vec<domain::share::event::base_domain_event::BaseDomainEvent<T>> {
        todo!()
    }

    fn save(&self, event: domain::share::event::base_domain_event::BaseDomainEvent<T>) {
        todo!()
    }

    fn update(&self, event: domain::share::event::base_domain_event::BaseDomainEvent<T>) {
        todo!()
    }
}
