use rocket::tokio::time::{sleep, Duration};
use rocket::serde::{Deserialize, json::Json};
use rocket::Rocket;
use rocket::Build;

use std::path::{Path, PathBuf};
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

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
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

#[get("/")]
fn root() -> &'static str {
    "Shane was here"
}

#[cfg(test)] mod test;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/files", routes![files])
        .mount("/", routes![wait, root, get_edit_dist, edit_dist_from_json])
}

