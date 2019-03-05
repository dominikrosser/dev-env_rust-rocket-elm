#![feature(custom_attribute)]   // Enable custom attributes like 'table_name'
//#![feature(plugin)]           //
//#![feature(proc_macro_derive)]  // Lets us apply procedural macros
#![feature(const_fn)]           // Allows us to define inline functions
#![feature(proc_macro_hygiene)] // TODO: This ist for rocket but what does this macro do?
#![feature(decl_macro)]         // Allows us to create declarative macros
#![allow(dead_code)]            // Disable warning for dead code


#[macro_use]                    //TODO: Remove imports, import with use only since rust 2018
extern crate diesel;            //TODO: Remove imports, import with use only since rust 2018
//extern crate r2d2;
//extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;            //TODO: Remove imports, import with use only since rust 2018
//extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod schema;         // Contains the db schema of diesel
mod models;         // Contains the db models and functionality to read, update, insert
mod db;             // Contains helpers for the database connection pool 
mod static_files;   // Containes api routes for serving static files

//use ::diesel::prelude::*;
//use ::diesel::pg::PgConnection;
use ::dotenv::dotenv;
use ::std::env;
//use ::rocket;
//use ::diesel;

fn init_rocket() -> rocket::Rocket {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let pool = db::init_pool(&database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/", routes![static_files::all, static_files::index])
}

fn launch_api() {
    init_rocket().launch();
}

fn main() {
    launch_api();
}
