use actix_web::{ App, HttpServer, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use actix_cors::Cors;

extern crate simple_logger;
use simple_logger::{SimpleLogger};
use log::LevelFilter;

pub mod forum_actions{
    pub mod create;
    pub mod cleaner;
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
    pub mod data;
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
    pub mod data;
    pub mod forum;
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
    cfg.service(web::resource("/checkuname&uname={uname}").route(web::get().to(get::auth::login::check_uname)));
    cfg.service(web::resource("/checkemail&email={email}").route(web::get().to(get::auth::login::check_email)));

    //forum specific

    //check if category exists
    cfg.service(web::resource("/checkcat&cat={cat}").route(web::get().to(get::forum::check_cat)));
    

    //create data
    cfg.service(web::resource("/newcategory").route(web::post().to(post::post::make_cat)));
    cfg.service(web::resource("/newpost").route(web::post().to(post::post::make_post)));
    cfg.service(web::resource("/newcomment").route(web::post().to(post::post::make_comment)));

    //get data
    cfg.service(web::resource("/get/bulk/{offset}").route(web::get().to(get::data::get_categories)));
    cfg.service(web::resource("/get/bulk/{offset}/{cat_id}").route(web::get().to(get::data::get_posts)));
    cfg.service(web::resource("/get/bulk/{offset}/{cat_id}/{post_id}").route(web::get().to(get::data::get_comments_no_parent)));
    cfg.service(web::resource("/get/bulk/{offset}/{cat_id}/{post_id}/{parent_id}").route(web::get().to(get::data::get_comments)));
    cfg.service(web::resource("/get/single/{post_id}").route(web::get().to(get::data::get_post)));


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
        let cors = Cors::new()
        .supports_credentials()
            .supports_credentials()
            .max_age(3600).finish();

        App::new()
            .wrap(cors)
            .configure(general_routes)
            
    })
        .workers(20)
        .keep_alive(15)
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
