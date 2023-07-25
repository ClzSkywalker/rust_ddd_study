use base::model::ddd::marker_interface::MarkerInterface;
use chrono::{DateTime, Local};

use super::{domain_event_enum::DomainEventEnum, enent_status_enum::EventStatusEnum};

pub trait IBaseDomainEvent: MarkerInterface {
    fn handle_success(&mut self);
    fn handle_failed(&mut self);
}

pub struct BaseDomainEvent<T>
where
    T: Send + Sync,
{
    // 幂等键:即为当前事件的id
    pub id: String,
    // 领域对象id
    pub domain_id: String,
    // 事件状态
    pub event_status: EventStatusEnum,
    pub event_type: DomainEventEnum,
    pub occurred_on: DateTime<Local>,
    pub data: T,
}

impl<T> MarkerInterface for BaseDomainEvent<T> where T: Send + Sync {}

impl<T> IBaseDomainEvent for BaseDomainEvent<T>
where
    T: Send + Sync,
{
    fn handle_success(&mut self) {
        self.event_status = EventStatusEnum::Success;
    }

    fn handle_failed(&mut self) {
        self.event_status = EventStatusEnum::Failure
    }
}
