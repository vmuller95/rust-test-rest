use std::io;
use actix_web::{
    web::{self},
    App, HttpServer, guard
};
use std::fs;
use futures::executor;
use std::{sync::mpsc, thread};

mod handlers;
use crate::handlers::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let (tx, rx) = mpsc::channel::<()>();
     
    fs::create_dir("images/");
    fs::create_dir("previews/");
    let server = HttpServer::new(|| App::new()
            .route("/load_image", web::put().to(load_image_mp))
            .route("/load_image", web::post().guard(guard::Header("content-type", "application/json")).to(load_image_json))
            .route("/load_image", web::post().guard(guard::Header("content-type", "application/x-www-form-urlencoded")).to(load_image_url)))
        .bind("127.0.0.1:9999")?
        .run();
        
    let srv = server.clone();
    thread::spawn(move || {
        // wait for shutdown signal
        rx.recv().unwrap();

        // stop server gracefully
        executor::block_on(srv.stop(true))
    });
        
    server.await
}
