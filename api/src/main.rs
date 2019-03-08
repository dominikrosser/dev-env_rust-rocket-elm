#![feature(custom_attribute)]   // Enable custom attributes like 'table_name'
#![feature(const_fn)]           // Allows us to define inline functions
#![feature(proc_macro_hygiene)] // TODO: This ist for rocket but what does this macro do?
#![feature(decl_macro)]         // Allows us to create declarative macros
#![allow(dead_code)]            // Disable warning for dead code


#[macro_use]                    //TODO: Remove imports, import with use only since rust 2018
extern crate diesel;            //TODO: Remove imports, import with use only since rust 2018
#[macro_use]
extern crate rocket;            //TODO: Remove imports, import with use only since rust 2018
#[macro_use]
extern crate serde_derive;      //TODO: Remove imports, import with use only since rust 2018
extern crate serde_json;        //TODO: Remove imports, import with use only since rust 2018
#[macro_use]
extern crate rocket_contrib;    //TODO: Remove imports, import with use only since rust 2018

mod schema;         // Contains the db schema of diesel
mod models;         // Contains the db models and functionality to read, update, insert
mod db;             // Contains helpers for the database connection pool 
mod static_files;   // Contains api routes for serving static files
mod routes;         // Contains api routs that serve json

use ::dotenv::dotenv;
use ::std::env;

use crate::routes::*;

fn init_rocket() -> rocket::Rocket {
    // Read DATABASE_URL from .env
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    // Init postgres connection pool managed by r2d2
    let pool = db::init_pool(&database_url);
    // Start up the rocket api / server
    rocket::ignite()
        // Use the above created r2d2 postgres connection pool
        .manage(pool)
        // Mount the api routes
        .mount(
            "/api/v1/",
            routes![routes::index, routes::new, routes::show, routes::delete, routes::update]
        )
        // Mount all the other routes to serve frontent SPA or static files
        .mount(
            "/",
            routes![static_files::all, static_files::index]
        )
        // Catch every other rout with a 404
        .register(catchers![routes::not_found])
}

fn launch_api_and_app() {
    init_rocket().launch();
}

fn main() {
    launch_api_and_app();
}
