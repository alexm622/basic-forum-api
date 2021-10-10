use actix_web::{web, HttpResponse};
use crate::structs::responses::get::Exists;
use crate::structs::requests::get::CatExistsCheck;
use crate::database::get;

//handle a username check request
pub async fn check_uname(path: web::Path<CatExistsCheck>) -> HttpResponse {
    log::info!("uname check handler");
    //get the remote ip address
    let exists:bool = get::check_cat_str(path.cat.to_owned()).unwrap();
    let response:Exists = Exists{
        exists: exists,
    };

    HttpResponse::Ok().json(response)
}
