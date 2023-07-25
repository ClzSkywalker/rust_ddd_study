use base::model::ddd::marker_interface::MarkerInterface;
use domain::share::event::{
    base_domain_event::IBaseDomainEvent, idomain_event_publisher::IDomainEventPublisher,
    idomain_event_repository::IDomainEventRespository,
};

pub struct DomainEventPushlisher<DER, T>
where
    DER: IDomainEventRespository<T>,
    T: Sync + Send,
{
    pub domain_event_repository: DER,
    _t: Option<T>, // 没有这个T会报错，无引用
}

impl<DER, DATA> MarkerInterface for DomainEventPushlisher<DER, DATA>
where
    DATA: Sync + Send,
    DER: IDomainEventRespository<DATA>,
{
}

impl<DER, T, DATA> IDomainEventPublisher<T> for DomainEventPushlisher<DER, DATA>
where
    DATA: Sync + Send,
    T: IBaseDomainEvent,
    DER: IDomainEventRespository<DATA>,
{
    fn publish(&self, event: T) {
        todo!()
    }

    fn publish_and_save(&self, event: T) {
        todo!()
    }
}
