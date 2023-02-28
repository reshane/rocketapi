
use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::{Status, ContentType};
use rocket::serde::Serialize;
use std::string::String;

use crate::edit_dist::edit_dist;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct PhrasePair {
    phrase1: String,
    phrase2: String,
}

impl PhrasePair {
    fn new(phrase1: String, phrase2: String) -> Self {
        Self {
            phrase1: phrase1,
            phrase2: phrase2,
        }
    }
}

#[test]
fn root() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!(super::root)).dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn edit_dist_post() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let phrase1: String = String::from("shane");
    let phrase2: String = String::from("who");
    let diff: usize = edit_dist(&phrase1.clone(), &phrase2.clone());
    let phrases = PhrasePair::new(phrase1, phrase2);
    let response = client
        .post("/shane")
        .json(&phrases)
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), diff.to_string());
}

#[test]
fn get_edit_dist() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let phrase1: String = String::from("shane");
    let phrase2: String = String::from("who");
    let diff: usize = edit_dist(&phrase1.clone(), &phrase2.clone());
    let response = client
        .get(format!("/shane/{}/{}", &phrase1, &phrase2)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), format!("{}", diff)); 
}

#[test]
fn who() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client
        .get("/whoami")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), (&"a nonce").to_string());
}
