use std::error::Error;

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
use askama_actix::Template;
use std::env;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[derive(Template)]
#[template(path = "pomodoro.html")]
struct PomodoroTemplate;

#[derive(Template)]
#[template(path = "bugle_sound.html")]
struct BugleSoundTemplate;

#[get("/")]
async fn index() -> impl Responder {
    Index
}

#[get("/app")]
async fn pomodoro() -> impl Responder {
    PomodoroTemplate
}

#[get("/bugle-sound")]
async fn bugle_sound() -> impl Responder {
    BugleSoundTemplate
}

#[get("/audio/{filename}")]
async fn get_audio(path: web::Path<String>) -> Result<NamedFile, Box<dyn Error>> {
    let filename = path.into_inner();
    let base_path = env::current_dir()?.join("assets");
    let file_path = base_path.join(filename);
    let file = NamedFile::open(file_path)?;
    Ok(file.use_last_modified(true))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().service(index).service(
            web::scope("/pomodoro")
                .service(pomodoro)
                .service(bugle_sound)
                .service(get_audio),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
