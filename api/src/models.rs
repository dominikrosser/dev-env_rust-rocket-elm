use ::diesel;
use ::diesel::prelude::*;
use ::diesel::pg::PgConnection;
use crate::schema::posts;
use crate::schema::posts::dsl::posts as all_posts;

//use serde_derive;
//use serde_json;

#[derive(Queryable)]    // Queryable from db table
#[derive(Serialize)]    // Convertable to json
#[derive(Debug)]        // Convertable to debug string
//#[derive(Clone)]      // Cloneable / copyable
pub struct Post {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
}

#[derive(Insertable)]   // Insertable to db table
#[derive(Serialize)]    // Convertable to json
#[derive(Deserialize)]  // Convertable from json
#[table_name = "posts"] // Table name: "posts"
pub struct NewPost<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
}

/** Example CRUD */
impl Post {

    pub fn get_by_id(id: i32, conn: &PgConnection) -> Vec<Post> {
        all_posts
            .find(id)
            .load::<Post>(conn)
            .expect("Error loading post")
    }

    pub fn all(conn: &PgConnection) -> Vec<Post> {
        all_posts
            .order(posts::id.desc())
            .load::<Post>(conn)
            .expect("Error loading the posts")
    }

    pub fn update_by_id(id: i32, conn: &PgConnection, post: NewPost) -> bool {
        use posts::dsl::{title as t, subtitle as s};
        let NewPost {
            title,
            subtitle
        } = post;
        diesel::update(all_posts.find(id))
            .set((t.eq(title), s.eq(subtitle)))
            .get_result::<Post>(conn)
            .is_ok()
    }

    pub fn insert(post: NewPost, conn: &PgConnection) -> bool {
        diesel::insert_into(posts::table)
            .values(&post)
            .execute(conn)
            .is_ok()
    }
    
    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if Post::get_by_id(id, conn).is_empty() {
            return false;
        };
        diesel::delete(all_posts.find(id)).execute(conn).is_ok()
    }
}
