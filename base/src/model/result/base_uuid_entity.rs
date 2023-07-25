use chrono::{DateTime, Local};

pub struct BaseUuidEntity {
    pub id: i32,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
}
