#![feature(custom_attribute, custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde;
extern crate serde_json;

mod entity;
mod error;

use entity::ZipResult;
use error::ZipError;
use hyper::Client;
use std::io::Read;

fn main() {
    let result = std::env::args().nth(1).ok_or(ZipError::Input)
        .and_then(|candidate| query(candidate))
        .and_then(|(candidate, response)| parse_result(candidate, response));

    match result {
        Ok(zip_result) => println!("{}", zip_result),
        Err(e) => println!("{}", e),
    }
}

fn query(candidate: String) -> Result<(String, String), ZipError> {
    match Client::new().get(&format!("http://api.zippopotam.us/us/{}", candidate)).send() {
        Ok(mut res) => Ok((candidate, read_response(&mut res))),
        _ => Err(ZipError::API),
    }
}

#[allow(unused)]
fn read_response(response: &mut hyper::client::Response) -> String {
    let mut buf = String::new();
    response.read_to_string(&mut buf);
    buf
}

fn parse_result(candidate: String, result: String) -> Result<ZipResult, ZipError> {
    match serde_json::from_str(&result) {
        Ok(result) => Ok(result),
        Err(_) => Err(ZipError::Invalid(candidate)),
    }
}
