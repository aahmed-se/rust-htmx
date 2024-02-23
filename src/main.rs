use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::atomic::{AtomicI32, Ordering};

static COUNTER: AtomicI32 = AtomicI32::new(0);

async fn increment() -> impl Responder {
    let new_count = COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    HttpResponse::Ok().body(format!("{}", new_count))
}

async fn decrement() -> impl Responder {
    let new_count = COUNTER.fetch_sub(1, Ordering::SeqCst) - 1;
    HttpResponse::Ok().body(format!("{}", new_count))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/increment", web::post().to(increment))
            .route("/decrement", web::post().to(decrement))
            .service(actix_files::Files::new("/", "./static")
                .index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
