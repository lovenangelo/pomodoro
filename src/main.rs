use std::error::Error;

use actix_files::{Files, NamedFile};
use actix_web::{get, web, App, HttpServer, Responder};
use askama_actix::Template;
use std::env;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[derive(Template)]
#[template(path = "pomodoro.html")]
struct PomodoroTemplate;

#[derive(Template)]
#[template(path = "sound.html")]
struct SoundTemplate<'a> {
    path: &'a str,
    id: &'a str,
}

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
    SoundTemplate {
        id: "bugle",
        path: "/pomodoro/audio/reveille.mp3",
    }
}

#[get("/button-sound-1")]
async fn button_sound_1() -> impl Responder {
    SoundTemplate {
        id: "button1",
        path: "/pomodoro/audio/button-sound-1.mp3",
    }
}

#[get("/button-sound-2")]
async fn button_sound_2() -> impl Responder {
    SoundTemplate {
        id: "button2",
        path: "/pomodoro/audio/button-sound-2.mp3",
    }
}

#[get("/ding-sound")]
async fn ding_sound() -> impl Responder {
    SoundTemplate {
        id: "ding",
        path: "/pomodoro/audio/ding.mp3",
    }
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
                .service(button_sound_1)
                .service(button_sound_2)
                .service(ding_sound)
                .service(get_audio)
                .service(Files::new("/styles", "./styles").show_files_listing()),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
