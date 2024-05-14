use actix_web::{guard, web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/")
                    .guard(guard::Host("wwww.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("wwww") })),
            )
            .service(
                web::scope("/")
                    .guard(guard::Host("users.rust-lang.org"))
                    .route("/", web::to(|| async { HttpResponse::Ok().body("user") })),
            )
            .route("/", web::to(HttpResponse::Ok))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
