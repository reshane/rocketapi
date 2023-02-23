use rocket::tokio::time::{sleep, Duration};
use std::io;
use rocket::tokio::task::spawn_blocking;

use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;

#[macro_use] extern crate rocket;

#[get("/")]
fn root() -> &'static str {
    "Shane was here"
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

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/hello", routes![world, wait, hello])
        .mount("/files", routes![files])
        .mount("/", routes![root, blocking_task])
        .launch()
        .await?;

    Ok(())
}
