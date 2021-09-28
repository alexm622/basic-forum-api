use actix_web::{ App, HttpServer, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

extern crate simple_logger;
use simple_logger::{SimpleLogger};
use log::LevelFilter;

pub mod auth{
    pub mod user;
}
pub mod database{
    pub mod insert;
}
pub mod structs{
    pub mod auth;
    pub mod comments;
    pub mod posts;
    pub mod categories;
    pub mod user;
    pub mod requests;
    pub mod responses;
    pub mod moderation;
    pub mod database;
}
pub mod get{
    pub mod auth;
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub name: String,
}

// Configure route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
    cfg.route("/posttest", web::post().to(post_test_handler));

    cfg.service(web::resource("/login&uname={uname}&pw={pw}").route(web::get().to(get::auth::login::login_handler)));
    
    /*cfg.service(web::resource("/redis&key={key}").route(web::get().to(db_request_handlers::redis_get_handler)));
    cfg.service(web::resource("/spotify&token={token}").route(web::get().to(spotify_api::spotify_generic)));
    cfg.route("/next", web::post().to(spotify_api::next_track));*/
}

//post request handler
pub async fn post_test_handler (req: web::Json<Request>) -> HttpResponse {
    HttpResponse::Ok().json(req.name.to_string())
}
/**
 * 
 */
pub async fn health_check_handler() ->  impl Responder {
    HttpResponse::Ok().json("Rust Server is running properly")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    SimpleLogger::new()
    .with_level(LevelFilter::Off)
    .with_module_level("rest", LevelFilter::Info)
    .with_module_level("actix", LevelFilter::Info)
    .init()
    .unwrap();


    
    HttpServer::new(|| {
        App::new()
            .configure(general_routes)
    })
        .workers(20)
        .keep_alive(15)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
