use crate::db::Conn as DbConn;
use crate::models::{Post, NewPost};
use ::rocket_contrib::json::Json;
use ::serde_json::Value;

#[get("/posts", format = "application/json")]
fn index(conn: DbConn) -> Json<Value> {
    let posts = Post::all(&conn);

    Json(json!({
        "status": 200,
        "result": posts,
    }))
}

#[post("/posts", format= "application/json", data = "<new_post>")]
fn new(conn: DbConn, new_post: Json<NewPost>) -> Json<Value> {
    Json(json!({
        "status": Post::insert(new_post.into_inner(), &conn),
        "result": Post::all(&conn).first(),
    }))
}

#[get("/posts/<id>", format = "application/json")]
fn show(conn: DbConn, id: i32) -> Json<Value> {
   let result = Post::get_by_id(id, &conn);
   let status = if result.is_empty() { 404 } else { 200 };

   Json(json!({
       "status": status,
       "result": result.get(0),
    }))
}

#[put("/posts/<id>", format = "application/json", data = "<post>")]
fn update(conn: DbConn, id: i32, post: Json<NewPost>) -> Json<Value> {
    let status = if Post::update_by_id(id, &conn, post.into_inner()) { 200 } else { 404 };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/posts/<id>")]
fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status = if Post::delete_by_id(id, &conn) { 200 } else { 404 };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

/** Sample get with select */
//#[get("/books/authors/<author>", format = "application/json")]
//fn author(author: String, conn: DbConn) -> Json<Value> {
//    Json(json!({
//        "status": "200",
//        "result": Book::all_by_author(author, &conn),
//    }))
//}

#[catch(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found",
    }))
}
