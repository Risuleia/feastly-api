use tokio::time::{ sleep, Duration };
use std::env;
use dotenv::dotenv;
use tokio;
use actix_web::{
    web,
    App,
    HttpServer,
    middleware::Logger,
    http
};
use actix_cors::Cors;

mod fetch_data;
mod db;
mod api;
mod api_structs;
mod models;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    let access_token = env::var("API_KEY").expect("API_KEY not set");

    let database = db::connect().await;
    let database2 = database.clone();
    
    env_logger::init();
    
    let server = HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .supports_credentials()
            )
            .wrap(logger)
            .app_data(web::Data::new(database.clone()))
            .service(web::resource("/recipes").to(api::get_data))
            .service(web::resource("/recipe").to(api::get_data))
    })
    .bind(("0.0.0.0", 8000))?
    .run();

    tokio::spawn(server);

    loop {
        sleep(Duration::from_secs(43200)).await;
        fetch_data::fetch_and_store_recipes(&access_token, &database2).await;
    }

}