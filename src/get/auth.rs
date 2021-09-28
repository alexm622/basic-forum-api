pub mod login{
    use actix_web::{ dev,web, HttpResponse};
    use serde::{Serialize, Deserialize};
    use crate::structs::requests::post::{Login, NewUser};
    use crate::structs::responses::post::{LoginResponse};

    

    pub async fn login_handler(path: web::Path<Login>) -> HttpResponse {
        
        log::info!("login handler");
        let uname = path.uname.clone();
        let pw = path.pw.clone();
        log::info!("uname: {}", uname);
        log::info!("password: {}", pw);

        
        HttpResponse::Ok().json(LoginResponse {outcome: false,login_token: None, uid: None})
        

    }
    pub async fn new_user_handler(path: web::Path<NewUser>,req: web::HttpRequest) -> HttpResponse {
        
        log::info!("redis get handler");
        let uname = path.uname.clone();
        let pw = path.pw.clone();
        let email = path.email.clone();
        let ip = req.connection_info().remote().unwrap().to_owned();
        log::info!("uname: {}", uname);
        log::info!("password: {}", pw);
        log::info!{"email: {}", email};
        log::info!{"ip: {}", ip};

        
        HttpResponse::Ok().json(LoginResponse {outcome: false,login_token: None, uid: None})
        

    }
}