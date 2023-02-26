
use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::Status;
use std::string::String;

use crate::edit_dist::edit_dist;

#[test]
fn root() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!(super::root)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Shane was here");
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
fn get_wait_time() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let seconds: u8 = 1;
    let message: String = format!("Waited {} seconds", seconds);
    let response = client
        .get(format!("/wait/{}", seconds))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), message);

}

#[test]
fn get_message() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let message: String = (&"ShaneWasHere").to_string();
    let response = client
        .get(format!("/echo/{}", message.clone()))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), message);
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
