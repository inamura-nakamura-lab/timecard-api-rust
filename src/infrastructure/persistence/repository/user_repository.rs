use crate::domain::repository::user_repository::{UserRepository, UsesUserRepository};
use crate::infrastructure::persistence::model::user::NewUser;
use crate::infrastructure::persistence::schema::user::schema;
use futures::Future;
use infrastructure::persistence::model::user::User;
use std::error::Error;

trait UserRepositoryImpl {}

impl<T: UserRepositoryImpl> UserRepository for T {
    fn insert_user(&self, new_user: &NewUser) -> Box<Future<Item = Option<bool>, Error = Error>> {
        diesel::insert_into(users::table);
    }
    fn select_user(&self, user_id: &i32) -> Box<Future<Item = Option<User>, Error = Error>> {}
    fn delete_user(&self, user_id: &i32) -> Box<Future<Item = Option<bool>, Error = Error>> {}
}
