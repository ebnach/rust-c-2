//! This is a documentation comment for the `module_1` crate.
//! 

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world22!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // lets create web server
    
    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
