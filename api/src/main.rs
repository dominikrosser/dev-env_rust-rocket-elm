#![feature(custom_attribute)]   // Enable custom attributes like 'table_name'
#![allow(dead_code)]            // Disable warning for dead code

#[macro_use]                    //TODO: Remove imports, import with use only since rust 2018
extern crate diesel;            //TODO: Remove imports, import with use only since rust 2018

mod schema;// Contains the db schema of diesel
mod models;// Contains the db models and functionality to read, update, insert

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

fn main() {

    // Read DATABASE_URL from .env
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    // Establish db connection
    let conn = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
    // Insert new post
    let post = crate::models::NewPost {
        title: "Hallo Welt Tutorial Post",
        subtitle: "Dies ist der Untertitel",
    };
    if crate::models::Post::insert(post, &conn) {
        println!("success");
    } else {
        println!("failure");
    }
}
