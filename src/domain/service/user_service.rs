use crate::domain::model::user::User;
use crate::domain::repository::user_repository::UsesUserRepository;
use futures::Future;
use std::error::Error;

trait UserService: UsesUserRepository {
    fn add_user(&self, new_user: &User) -> Box<Future<Item = Option<bool>, Error = String>>;
    fn get_user(&self, user_id: &i32) -> Box<Future<Item = Option<User>, Error = String>>;
    fn delete_user(&self, user_id: &i32) -> Box<Future<Item = Option<bool>, Error = String>>;
}

trait UsesUserService {
    type UserService: UserService;
    fn user_service(&self) -> Self::UserService;
}

impl<T: UsesUserRepository> UserService for T {
    fn add_user(&self, new_user: &User) -> Box<Future<Item = Option<bool>, Error = String>> {
        self.user_repository().insert_user();
    }
    fn get_user(&self, user_id: &i32) -> Box<Future<Item = Option<User>, Error = String>> {}
    fn delete_user(&self, user_id: &i32) -> Box<Future<Item = Option<bool>, Error = String>> {}
}
