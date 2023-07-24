use base::model::ddd::repository::Repository;
use domain::aggregate::user::{repository::user_repository::{self, IUserRepository}, model::user::User};

pub struct UserRepositoryImpl{

}

impl Repository<User,String> for UserRepositoryImpl {
    fn delete(id: String) {
        todo!()
    }

    fn by_id(id: String) -> User {
        todo!()
    }

    fn save(s: User) -> User {
        todo!()
    }

    fn save_and_flush(s: User) -> User {
        todo!()
    }
}

impl IUserRepository for UserRepositoryImpl{
    fn by_user_name(user_name: String) -> domain::aggregate::user::model::user::User {
        todo!()
    }
}