use actix_web::{web, HttpResponse};
use crate::structs::responses::get::Exists;
use crate::structs::requests::get::CatExistsCheck;
use crate::database::get;

//handle a username check request
pub async fn check_cat(path: web::Path<CatExistsCheck>) -> HttpResponse {
    log::info!("category check handler");
    //get the remote ip address
    let exists:bool = get::cat_exists(path.cat.to_owned()).unwrap();
    let response:Exists = Exists{
        exists: exists,
    };

    HttpResponse::Ok().json(response)
}
