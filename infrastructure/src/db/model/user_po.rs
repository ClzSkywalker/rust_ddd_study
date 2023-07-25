use base::model::result::base_uuid_entity::BaseUuidEntity;

pub struct UserPo{
    pub base:BaseUuidEntity,
    pub user_name:String,
    pub real_name:String,
    pub phone:String,
    pub password:String,
    pub unit_id:i32,
    pub roleIds:Vec<i32>,
    pub provence:String,
    pub city:String,
    pub county:String,
}