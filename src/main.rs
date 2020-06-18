use std::io;
use actix_web::{
    web::{self},
    App, HttpServer, guard
};
use std::fs;

mod handlers;
use crate::handlers::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    fs::create_dir("images/");
    fs::create_dir("previews/");
    HttpServer::new(|| App::new()
            .route("/load_image", web::put().to(load_image_mp))
            .route("/load_image", web::post().guard(guard::Header("content-type", "application/json")).to(load_image_json))
            .route("/load_image", web::post().guard(guard::Header("content-type", "application/x-www-form-urlencoded")).to(load_image_url)))
        .bind("127.0.0.1:9999")?
        .run()
        .await
}
