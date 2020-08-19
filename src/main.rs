#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate serde;

#[macro_use]
extern crate diesel;
use diesel::prelude::*;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub mod models;
use models::*;
pub mod schema;

#[database("sqlite_logs")]
pub struct LogsDbConn(diesel::SqliteConnection);

fn main() {
    rocket::ignite()
        .attach(LogsDbConn::fairing())
        .mount("/workloads", routes![workload_create, workload_read])
        .launch();
}

use rocket_contrib::json::Json;

#[post("/new", format = "application/json", data = "<workload>")]
pub fn workload_create(conn: LogsDbConn, workload: Json<NewWorkload>) -> Option<Json<Workload>> {
    use schema::workloads::dsl::*;
    let o_id = workload.id.as_ref()?;
    println!("Workload id: {}", o_id);

    let res = Workload {
        id: "hello".to_string(),
        repo_id: workload.repo_id.clone(),
        repo_version: workload.repo_version.clone(),
    };
    diesel::insert_into(workloads).values(&res).execute(&*conn);
    Some(Json(res))
}

#[get("/")]
pub fn workload_read(conn: LogsDbConn) -> QueryResult<Json<Vec<Workload>>> {
    use schema::workloads::dsl::*;
    Ok(Json(workloads.load::<Workload>(&*conn)?))
}
