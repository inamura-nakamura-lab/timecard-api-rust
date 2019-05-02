use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: Option<i32>,
    name: Option<String>,
    student_number: Option<String>,
    date: SystemTime
}