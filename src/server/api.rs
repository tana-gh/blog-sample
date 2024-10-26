use actix_web::{
    get, web, HttpResponse, Responder
};
use diesel::prelude::*;
use crate::db;
use crate::json;

pub fn api_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
        .service(blog_post)
    );
}

#[get("/post/{post_id}")]
async fn blog_post(post_id: web::Path<i32>) -> impl Responder {
    let post_id = post_id.into_inner();
    let result =
        db::models::Post::first(|posts| posts.find(post_id));
    match result {
        Ok(post) =>
            HttpResponse::Ok().json(json::Post::from(post)),
        Err(_) =>
            HttpResponse::NotFound().finish(),
    }
}
