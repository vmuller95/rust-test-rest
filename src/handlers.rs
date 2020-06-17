extern crate serde_qs as qs;
extern crate base64;
extern crate crypto;
extern crate percent_encoding;

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
    
    let mut hasher = Sha3::sha3_256();

    // write input message
    hasher.input(&image_data);
    
    // read hash digest
    let hex = hasher.result_str();
    
    write_to_fle(&format!("images/{}", hex), &image_data);
    
    
    StatusCode::OK

}

fn handle_json(json_data: &str) -> StatusCode {
    StatusCode::OK
}

fn handle_form_urlencoded(bytes: Bytes) -> StatusCode {
    let form_result = qs::from_bytes::<FormData>(&bytes);
    let form = match form_result {
        Ok(form_data) => form_data,
        Err(_) => return StatusCode::BAD_REQUEST
    };
    
    
    
    if form.params.len() != form.upload_types.len() {
        return StatusCode::BAD_REQUEST;
    }
    
    for i in 0 .. form.params.len() {
        let code = match &(form.upload_types[i][..]) {
            "base64" => handle_base64(&form.params[i]),
            "json" => handle_json(&form.params[i]),
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
        "application/x-www-form-urlencoded" => handle_form_urlencoded(bytes),
        _ => StatusCode::NOT_FOUND
    };
    
    HttpResponse::build(code).finish()
}