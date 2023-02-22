//AUTHOR: Emil Lantz 2023

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    HttpServer::new(|| {
        App::new()
            .service(routes::test::test)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
