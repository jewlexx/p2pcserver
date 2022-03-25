#![feature(rustc_private)]

use anyhow::Context as _;
use rocket::{response::status::NotFound, routes, serde::json::Json};
use serde::Serialize;
use std::{
    fs::File,
    io::{Read, Write},
};

#[macro_use]
extern crate rocket;

fn get_file_path() -> anyhow::Result<String> {
    let mut path = std::env::current_exe()?;
    path.pop();
    path.push("data.txt");
    let path = path.to_str().context("Could not convert path to string")?;

    Ok(path.to_string())
}

fn get_file() -> anyhow::Result<(File, File)> {
    let file_path = get_file_path().unwrap();

    let mut file;

    let file_read = match File::open(&file_path) {
        Ok(mut v) => {
            let mut file_data = String::new();
            v.read_to_string(&mut file_data)?;

            file = File::create(&file_path)?;
            file.write_all(file_data.as_bytes())?;

            File::open(&file_path)?
        }
        Err(_) => {
            file = File::create(&file_path)
                .context("Failed to create file. Double check permissions")?;

            File::open(&file_path)?
        }
    };

    Ok((file_read, file))
}

fn get_data() -> anyhow::Result<Vec<String>> {
    let mut file = get_file()?;

    let mut file_string = String::new();
    file.0.read_to_string(&mut file_string)?;

    let entries: Vec<String> = file_string.split('\n').map(|s| s.to_string()).collect();

    Ok(entries)
}

#[post("/", data = "<data>")]
async fn add_data(data: String) -> Result<String, NotFound<String>> {
    let str = format!("{}\n", data);
    let bytes = str.as_bytes();

    let mut file = match get_file() {
        Ok(v) => v,
        Err(_) => return Err(NotFound("Couldn't get the file sorry".to_string())),
    };

    file.1.write_all(bytes).unwrap();

    Ok(format!("Sending {} to \"database\"", data))
}

#[derive(Serialize)]
struct Entries {
    data: Vec<String>,
}

#[get("/")]
fn get_entries() -> Json<Entries> {
    let entries = get_data().unwrap_or_default();

    Json(Entries { data: entries })
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rocket = rocket::build()
        .mount("/", routes![get_entries, add_data])
        .ignite()
        .await?;

    rocket.launch().await
}
