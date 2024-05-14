use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key,pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.perm").unwrap();

    HttpServer::new(|| App::new().service(index))
        .bind_openssl("localost:8080", builder)?
        .run()
        .await
}
