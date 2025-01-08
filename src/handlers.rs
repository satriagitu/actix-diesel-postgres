// handlers.rs
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::actions::{find_user_by_uid, insert_new_user};
use crate::db::DbPool;
use crate::models::NewUser;
use uuid::Uuid;

#[get("/users/{id}")]
async fn get_users(pool: web::Data<DbPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    match find_user_by_uid(conn, user_id.into_inner()) {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/users")]
async fn create_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    match insert_new_user(conn, &new_user.name, &new_user.email) {
        Ok(user) => HttpResponse::Created().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
