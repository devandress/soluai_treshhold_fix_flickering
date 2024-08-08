use actix_cors::Cors;
use actix_web::{web::{self, scope}, App, HttpServer};
mod services;
mod yolo;
use services::index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = match std::env::var("PORT") {
        Ok(port) => port,
        _ => String::from("8000"),
    };

    let address = format!("0.0.0.0:{port}");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"ws://localhost")
            })
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"ws://127.0.0.1")
            })
            .allowed_methods(vec!["GET", "POST"]);
        App::new()
            .wrap(cors)
            .service(
                scope("/api")
                    .route("predict/", web::get().to(index))
            )
    })
    .bind(address.clone())?
    .run()
    .await
}