
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod controllers;
mod daos;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new( move || {
        App::new()
            .service( controllers::main_controller::health_handler )
            .service( controllers::main_controller::root )      
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
