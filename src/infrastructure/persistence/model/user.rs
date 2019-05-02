use std::time::SystemTime;
use infrastructure::persistence::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub student_number: String,
    pub date: SystemTime
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub student_number: &'a str,
}