use diesel::prelude::*;

#[derive(Queryable)]
pub struct Service {
    pub id: i32,
    pub name: String,
    pub secret_digest: String,
    pub password: Option<i32>,
    pub created_at: i32,
    pub last_used_at: Option<i32>,
}



#[derive(Queryable)]
pub struct Password {
    pub id: i32,
    pub salt: String,
    pub digest: String,
    pub created_at: i32,
}
