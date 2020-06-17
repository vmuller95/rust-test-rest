extern crate serde_qs as qs;
extern crate base64;
extern crate crypto;
extern crate percent_encoding;
extern crate reqwest;

use actix_web::*;
use actix_web::http::{StatusCode};
use actix_web::web::Bytes;
use serde::Deserialize;
use std::vec::Vec;
use actix_multipart::Multipart;
use base64::decode;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use std::io::prelude::*;
use std::fs::File;
use percent_encoding::percent_decode;
use actix_web::client::Client;




#[derive(Deserialize)]
pub struct FormData {
    params: Vec<String>,
    upload_types: Vec<String>
}

fn write_to_fle(fname: &str, data: &[u8]) {
    let mut pos = 0;
    let mut buffer = File::create(fname).unwrap();

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..]).unwrap();
        pos += bytes_written;
    }
}

fn get_hash(bytes: &[u8]) -> String{
    let mut hasher = Sha3::sha3_256();

    // write input message
    hasher.input(bytes);
    
    // read hash digest
    hasher.result_str()
    
}

fn handle_base64(base64_image: &str) -> StatusCode {
    let decode_result = percent_decode(base64_image.as_bytes()).decode_utf8();
    let base64_decoded = match decode_result {
        Ok(decoded) => decoded,
        Err(_) => return StatusCode::BAD_REQUEST
    };
    
    
    let image_data = match decode(&base64_decoded[..]) {
        Ok(bytes) => bytes,
        Err(_) => return StatusCode::BAD_REQUEST
    };
    
    let hex = get_hash(&image_data);
    
    write_to_fle(&format!("images/{}", hex), &image_data);
    
    
    StatusCode::OK

}

async fn  handle_uri(uri_str: &str) -> StatusCode {
   let mut client = Client::default();

   // Create request builder and send request
   let resp_res= reqwest::get(uri_str).await;
      
    let mut resp = match resp_res {
        Ok(ok_resp) => ok_resp,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR
    };
    
    if !resp.status().is_success() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    
    
    
    let body_result = resp.bytes().await;
    let body = match body_result {
        Ok(body_ok) => body_ok,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR
    };
    
    //let body_vec = body.to_vec();
    let hex = get_hash(&body);
    println!("HEX: {:?}", hex);
    
    write_to_fle(&format!("images/{}", hex), &body);

    StatusCode::OK
}

async fn handle_form_urlencoded(bytes: Bytes) -> StatusCode {
    let form_result = qs::from_bytes::<FormData>(&bytes);
    let form = match form_result {
        Ok(form_data) => form_data,
        Err(_) => return StatusCode::BAD_REQUEST
    };
    
    
    
    if form.params.len() != form.upload_types.len() {
        return StatusCode::BAD_REQUEST;
    }
    
    for i in 0 .. form.params.len() {
        println!("form.params[{}]={}", i, form.params[i]);
        let code = match &(form.upload_types[i][..]) {
            "base64" => handle_base64(&form.params[i]),
            "uri" => handle_uri(&form.params[i]).await,
            _ => StatusCode::BAD_REQUEST
        };
        
        if code != StatusCode::OK {
            return code;
        }
    } 
    
    StatusCode::OK
}

pub async fn load_image(bytes: Bytes, mut _payload: Multipart, req: HttpRequest) -> HttpResponse {    
   println!("{}", req.content_type());
    
    let code = match req.content_type() {
        "application/x-www-form-urlencoded" => handle_form_urlencoded(bytes).await,
        _ => StatusCode::NOT_FOUND
    };
    
    HttpResponse::build(code).finish()
}