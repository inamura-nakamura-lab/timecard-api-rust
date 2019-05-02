use crate::domain::model::user::User;
use crate::infrastructure::persistence::model::user::NewUser;
use futures::Future;
use std::error::Error;

pub trait UserRepository {
    fn insert_user(&self, new_user: &NewUser) -> Box<Future<Item = Option<bool>, Error = String>>;
    fn select_user(&self, user_id: &i32) -> Box<Future<Item = Option<Vec<User>>, Error = String>>;
    fn delete_user(&self, user_id: &i32) -> Box<Future<Item = Option<bool>, Error = String>>;
}

pub trait UsesUserRepository {
    type UserRepository: UserRepository;
    fn user_repository(&self) -> Self::UserRepository;
}
