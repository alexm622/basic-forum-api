use actix_web::{ App, HttpServer, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

extern crate simple_logger;
use simple_logger::{SimpleLogger};
use log::LevelFilter;

pub mod forum_actions{
    pub mod create;
}
pub mod utils{
    pub mod ip_tools;
}
pub mod auth{
    pub mod user;
}
pub mod post{
    pub mod post;
    pub mod auth;
}
pub mod database{
    pub mod insert;
    pub mod get;
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
    
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub name: String,
}

// Configure route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    //general purpose
    cfg.route("/health", web::get().to(health_check_handler));
    cfg.route("/posttest", web::post().to(post_test_handler));

    //auth
    cfg.service(web::resource("/login").route(web::post().to(post::auth::login::login_handler)));
    cfg.service(web::resource("/newuser").route(web::post().to(post::auth::login::new_user_handler)));

    //forum specific
    cfg.service(web::resource("/newpost").route(web::post().to(post::post::post::make_post)));
    cfg.service(web::resource("/newcategory").route(web::post().to(post::post::post::make_cat)));
    cfg.service(web::resource("/newcomment").route(web::post().to(post::post::post::make_comment)));

}

//post request handler
pub async fn post_test_handler (req: web::Json<Request>) -> HttpResponse {
    HttpResponse::Ok().json(req.name.to_string())
}
/**
 * 
 */
pub async fn health_check_handler() ->  impl Responder {
    let resp:structs::responses::post::StatusResponse = structs::responses::post::StatusResponse{
        redirect:None,
        response_code:100
    };
    log::info!("returning the health of the api server");
    HttpResponse::Ok().json(resp)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    SimpleLogger::new()
    .with_level(LevelFilter::Error)
    .with_module_level("basic_forum_api", LevelFilter::Info)
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
