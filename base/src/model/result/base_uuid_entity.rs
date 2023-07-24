use chrono::{DateTime, Local};

pub struct BaseUuidEntity {
    pub id: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
}
