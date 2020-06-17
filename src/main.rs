use std::io;
use actix_web::{
    web::{self},
    App, HttpServer
};
use std::fs;

mod handlers;
use crate::handlers::load_image;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    fs::create_dir("images/");
    fs::create_dir("previews/");
    HttpServer::new(|| App::new().route("/load_image", web::post().to(load_image)))
        .bind("127.0.0.1:9999")?
        .run()
        .await
}
