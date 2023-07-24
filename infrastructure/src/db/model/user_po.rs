use base::model::result::base_uuid_entity::BaseUuidEntity;

pub struct UserPo{
    pub base:BaseUuidEntity,
    pub user_name:String,
    pub real_name:String,
    pub phone:String,
    pub password:String,
    pub unit_id:String,
    pub roleIds:String,
    pub provence:String,
    pub city:String,
    pub county:String,
}