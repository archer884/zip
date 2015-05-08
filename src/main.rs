#![feature(plugin)]
#![plugin(regex_macros)]
extern crate hyper;
extern crate regex;
mod error;
use error::ZipError;
use hyper::Client;
use regex::Regex;
use std::io::Read;

static CITY: Regex = regex!("\"place name\": \"(.*?)\"");
static STATE: Regex = regex!("\"state\": \"(.*?)\"");

fn main() {
    let result = std::env::args().nth(1).ok_or(ZipError::Input)
        .and_then(|candidate| query(candidate))
        .and_then(|(candidate, response)| parse_result(candidate, response));

    match result {
        Ok((city, state, zip)) => println!("{}, {} {}", city, state, zip),
        Err(e) => println!("{}", e),
    }
}

fn query(candidate: String) -> Result<(String, String), ZipError> {
    let mut client = Client::new();

    match client.get(&format!("http://api.zippopotam.us/us/{}", candidate)).send() {
        Ok(mut res) => Ok((candidate, read_response(&mut res))),
        _ => Err(ZipError::API),
    }     
}

#[allow(unused)]
fn read_response(response: &mut hyper::client::response::Response) -> String {
    let mut buf = String::new();
    response.read_to_string(&mut buf);
    buf
}

fn parse_result(candidate: String, result: String) -> Result<(String, String, String), ZipError> {
    let city = CITY.captures(&result).and_then(|c| c.at(1));
    let state = STATE.captures(&result).and_then(|c| c.at(1));

    match (city, state) {
        (Some(city), Some(state)) => Ok((city.to_string(), state.to_string(), candidate)),
        _ => Err(ZipError::Invalid(candidate.to_string())),
    }
}
