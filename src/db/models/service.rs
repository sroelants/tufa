use diesel::prelude::*;
use services::dsl;
use crate::db::schema::services;
use crate::util::now;
use anyhow::Error;

#[derive(Queryable, Debug)]
pub struct Service {
    pub id: Option<i32>,
    pub name: String,
    pub secret: String,
    pub encrypted: i32,
    pub created_at: i32,
    pub last_used_at: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = services)]
pub struct NewService<'a> {
    name: &'a str,
    secret: &'a str,
    created_at: i32,
    encrypted: i32,
}

pub fn create(conn: &mut SqliteConnection, name: &str, secret: &str, encrypted: bool) -> Result<Service, Error> {
    let new_service = NewService { 
        name, 
        secret, 
        encrypted: if encrypted { 1 } else { 0 },
        created_at: now() as i32, 
    };

    diesel::insert_into(services::table)
        .values(&new_service)
        .get_result(conn)
        .map_err(|_| Error::msg("Failed to insert service into database."))
}

pub fn get_by_name(conn: &mut SqliteConnection, name: &str) -> Option<Service> {
    dsl::services.filter(dsl::name.eq(name)).first(conn).ok()
}

pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<Service>, Error> {
    dsl::services.get_results(conn)
        .map_err(|_| Error::msg("Failed to get services from database."))
}

pub fn remove(conn: &mut SqliteConnection, name: &str) -> Result<(), Error> {
    diesel::delete(dsl::services.filter(dsl::name.eq(name)))
        .execute(conn)
        .map_err(|_| Error::msg("Failed to remove service"))?;

    Ok(())
}
