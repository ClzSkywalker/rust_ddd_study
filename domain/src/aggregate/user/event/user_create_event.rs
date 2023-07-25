use chrono::Local;

use crate::{
    aggregate::user::model::user::User,
    share::event::{
        base_domain_event::BaseDomainEvent, domain_event_enum::DomainEventEnum,
        enent_status_enum::EventStatusEnum,
    },
};

// pub enum UserEvent {
//     UserCreateEvent(User),
//     Now,
// }

// impl UserEvent {
//     fn user(&self) -> BaseDomainEvent<User> {
//         match self {
//             UserEvent::UserCreateEvent(item) => BaseDomainEvent::<User> {
//                 id: uuid::Uuid::new_v4().to_string(),
//                 domain_id: item.id.to_string(),
//                 event_status: EventStatusEnum::Pending,
//                 event_type: DomainEventEnum::UserCreate,
//                 occurred_on: Local::now(),
//                 data: item.clone(),
//             },
//         }
//     }
// }

pub trait CreateEvent<T>
where
    T: Send + Sync,
{
    fn create(data: T) -> BaseDomainEvent<T>;
}

pub struct UserCreateEvent {
    pub event: BaseDomainEvent<User>,
}

impl CreateEvent<User> for UserCreateEvent {
    fn create(data: User) -> BaseDomainEvent<User> {
        BaseDomainEvent::<User> {
            id: uuid::Uuid::new_v4().to_string(),
            domain_id: data.id.to_string(),
            event_status: EventStatusEnum::Pending,
            event_type: DomainEventEnum::UserCreate,
            occurred_on: Local::now(),
            data: data.clone(),
        }
    }
}
