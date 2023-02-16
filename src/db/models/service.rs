use diesel::prelude::*;
use crate::db::schema::services;
use crate::util::now;

#[derive(Queryable, Debug)]
pub struct Service {
    pub id: Option<i32>,
    pub name: String,
    pub secret: String,
    pub created_at: i32,
    pub last_used_at: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = services)]
pub struct NewService<'a> {
    name: &'a str,
    secret: &'a str,
    created_at: i32,
}

pub fn create(conn: &mut SqliteConnection, name: &str, secret: &str) -> Service {
    let new_service = NewService { 
        name, 
        secret, 
        created_at: now() as i32, 
    };

    diesel::insert_into(services::table)
        .values(&new_service)
        .get_result(conn)
        .expect("Failed to add new service")
}

pub fn get_by_name(conn: &mut SqliteConnection, name: &str) -> Option<Service> {
    use services::dsl;
    dsl::services.filter(dsl::name.eq(name)).first(conn).ok()
}
