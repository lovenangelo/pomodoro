use actix_web::{get, web, App, HttpServer, Responder};
use askama_actix::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[derive(Template)]
#[template(path = "pomodoro.html")]
struct PomodoroTemplate;

#[get("/")]
async fn index() -> impl Responder {
    Index
}

#[get("/app")]
async fn pomodoro() -> impl Responder {
    PomodoroTemplate
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(web::scope("/pomodoro").service(pomodoro))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
