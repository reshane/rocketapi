use rocket::tokio::time::{sleep, Duration};
use rocket::serde::{Deserialize, json::Json};
use rocket::Rocket;
use rocket::Build;

use std::path::Path;
use rocket::fs::NamedFile;

mod edit_dist;

use crate::edit_dist::edit_dist;

#[macro_use] extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct PhrasePair {
    phrase1: String,
    phrase2: String,
}

#[post("/shane", data = "<phrases>")]
fn edit_dist_from_json(phrases: Json<PhrasePair>) -> String {
    format!("{}", edit_dist(&phrases.phrase1, &phrases.phrase2))
}

#[get("/csgodata")]
async fn files() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/files/csgo_round_snapshots.csv")).await.ok()
}

#[get("/wait/<seconds>")]
async fn wait(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited {} seconds", seconds)
}

#[get("/shane/<p1>/<p2>")]
fn get_edit_dist(p1: String, p2: String) -> String {
    format!("{}", edit_dist(&p1, &p2))
}

#[get("/echo/<message>")]
fn echo(message: String) -> String {
    message
}

#[get("/index.js")]
async fn script() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/site/index.js")).await.ok()
}

#[get("/")]
async fn root() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/site/index.html")).await.ok()
}

#[get("/whoami")]
fn whoami() -> &'static str {
    "a nonce"
}

#[cfg(test)] mod test;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![script, whoami, files, wait, root, echo, get_edit_dist, edit_dist_from_json])
}

