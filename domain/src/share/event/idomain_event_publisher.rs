use base::model::ddd::marker_interface::MarkerInterface;

use super::base_domain_event::IBaseDomainEvent;

// 领域事件发布接口
pub trait IDomainEventPublisher<T>: MarkerInterface
where
    T: IBaseDomainEvent,
{
    fn publish(&self, event: T);
    fn publish_and_save(&self, event: T);
}
