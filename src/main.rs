mod rss;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(rss::list())
}

#[get("/podcasts/{title}")]
async fn podcast_feed(title: web::Path<String>) -> std::io::Result<String> {
    Ok(format!("RSS feed for {} podcast", title))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(podcast_feed)
    })
    .bind(("127.0.0.1", 7777))?
    .run()
    .await
}