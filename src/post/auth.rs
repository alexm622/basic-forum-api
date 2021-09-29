pub mod login{
    use actix_web::{web, HttpResponse};
    use crate::structs::requests::post::{Login, NewUser};
    use crate::structs::responses::post::{LoginResponse};
    use crate::database::insert::insert;
    use crate::structs::database::database;
    use crate::auth::user::login;
    

    pub async fn login_handler(json: web::Json<Login>,req: web::HttpRequest) -> HttpResponse {
        
        log::info!("login handler");
        let uname = json.uname.clone();
        let pw = json.pw.clone();
        let ip = req.connection_info().remote().unwrap().to_owned();
        log::info!("uname: {}", uname);
        log::info!("password: {}", pw);

        let response:LoginResponse = login::login(json.into_inner(), ip);

        HttpResponse::Ok().json(response)
    }
    pub async fn new_user_handler(json: web::Json<NewUser>,req: web::HttpRequest) -> HttpResponse {
        
        log::info!("new user handler");
        let uname = json.uname.clone();
        let pw = json.pw.clone();
        let email = json.email.clone();
        let ip = req.connection_info().remote().unwrap().to_owned();
        log::info!("uname: {}", uname);
        log::info!("password: {}", pw);
        log::info!{"email: {}", email};
        log::info!{"ip: {}", ip};

        let newuser: database::NewUser = database::NewUser{
            email: email,
            pw: pw,
            ip:ip,
            username: uname
        };

        let out: String = insert::add_user(newuser).unwrap();
        
        HttpResponse::Ok().json(LoginResponse {outcome: false,login_token: Some(out), uid: None})
    }
}