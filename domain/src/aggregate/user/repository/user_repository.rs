use base::model::ddd::repository::Repository;

use crate::aggregate::user::model::user::User;

pub trait IUserRepository: Repository<User, String> {
    fn by_user_name(user_name: String) -> User;
}
