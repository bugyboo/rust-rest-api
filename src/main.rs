
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use model::AppState;

mod controllers;
mod daos;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let todo_db = AppState::init();
    let app_data = web::Data::new(todo_db);

    println!("🚀 Server started successfully");

    HttpServer::new( move || {
        App::new()
            .app_data( app_data.clone() )
            .service( controllers::main_controller::health_handler )
            .service( controllers::main_controller::root )
            .service( controllers::todo_controller::todo_list_handler )
            .service( controllers::todo_controller::create_todo_handler )   
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
