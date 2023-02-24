use rocket::tokio::time::{sleep, Duration};
use std::io;
use rocket::tokio::task::spawn_blocking;
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

#[post("/shane", data = "<task>")]
fn edit_dist_from_json(task: Json<PhrasePair>) -> String {
    format!("{}", edit_dist(&task.phrase1, &task.phrase2))
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, World!"
}

#[get("/wait/<seconds>")]
async fn wait(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited {} seconds", seconds)
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result::<Vec<u8>> {
    let vec = spawn_blocking(|| std::fs::read("static/csgo_round_snapshots.csv")).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[get("/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("{} is so cool and is {} years old", name, age)
    } else {
        format!("aw don't say that... just lie")
    }
}

#[get("/shane/<p1>/<p2>")]
fn get_edit_dist(p1: String, p2: String) -> String {
    format!("{}", edit_dist(&p1, &p2))
}

#[get("/")]
fn root() -> &'static str {
    "Shane was here"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/hello", routes![world, wait, hello])
        .mount("/files", routes![files])
        .mount("/", routes![root, get_edit_dist, edit_dist_from_json, blocking_task])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

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
        let phrase2: String = String::from("shane");
        let diff: usize = edit_dist(&phrase1.clone(), &phrase2.clone());
        let response = client
            .get(format!("/shane/{}/{}", &phrase1, &phrase2)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), format!("{}", diff)); 
    }
}
