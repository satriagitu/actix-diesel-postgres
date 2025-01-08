// handlers.rs
use actix_web::{error, get, post, web, HttpResponse, Responder};
use crate::actions::{find_user_by_uid, insert_new_user};
use crate::db::DbPool;
use crate::models::NewUser;
use uuid::Uuid;

#[get("/users/{id}")]
async fn get_users(pool: web::Data<DbPool>, user_id: web::Path<Uuid>) -> actix_web::Result<impl Responder> {
    // let conn = &mut pool.get().expect("Failed to get DB connection");
    let user_uid = user_id.into_inner();
    let user = web::block(move || {
        let mut conn = pool.get()?; // Dapatkan koneksi dari pool
        find_user_by_uid(&mut conn, user_uid) // Masukkan data user ke database
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match user {
        // user was found; return 200 response with JSON formatted user object
        Some(user) => HttpResponse::Ok().json(user),

        // user was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No user found with UID: {user_uid}")),
    })
}

#[post("/users")]
async fn create_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>,
) -> actix_web::Result<impl Responder> {
    let new_user = new_user.into_inner(); // Konversi Json ke struct NewUser

    // Memindahkan operasi ke thread pool untuk mencegah blocking
    let user = web::block(move || {
        let mut conn = pool.get()?; // Dapatkan koneksi dari pool
        insert_new_user(&mut conn, &new_user.name, &new_user.email) // Masukkan data user ke database
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // user was added successfully; return 201 response with new user info
    Ok(HttpResponse::Created().json(user))
}