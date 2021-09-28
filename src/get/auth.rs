pub mod login{
    use actix_web::{ web, HttpResponse};
    use serde::{Serialize, Deserialize};
    use crate::structs::responses::post::LoginResponse;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Login {
        pub uname: String,
        pub pw: String,
    }

    pub async fn login_handler(path: web::Path<Login>) -> HttpResponse {
        
        log::info!("redis get handler");
        let uname = path.uname.clone();
        let pw = path.pw.clone();
        log::info!("uname: {}", uname);
        log::info!("password: {}", pw);

        
        HttpResponse::Ok().json(LoginResponse {outcome: false,login_token: None, uid: None})
        

    }
}