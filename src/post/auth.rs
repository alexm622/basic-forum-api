pub mod login{
    use actix_web::{web, HttpResponse};
    use crate::structs::requests::post::{Login, NewUser};
    use crate::structs::responses::post::{LoginResponse, NewUserResponse};
    use crate::database::insert;
    use crate::structs::database;
    use crate::auth::user::login;
    use crate::forum_actions::cleaner;
    
    //handle a login post request
    pub async fn login_handler(json: web::Json<Login>,req: web::HttpRequest) -> HttpResponse {
        log::info!("login handler");
        //get the remote ip address
        let ip = req.connection_info().remote_addr().unwrap().to_owned();
        //attempt to login
        let response:LoginResponse = login::login(json.into_inner(), ip);
        //return the response
        HttpResponse::Ok().json(response)
    }

    //create a new user handler
    pub async fn new_user_handler(json: web::Json<NewUser>,req: web::HttpRequest) -> HttpResponse {
        log::info!("new user handler");
        let uname = cleaner::clean_full(json.uname.clone());
        let pw = json.pw.clone();
        let email = json.email.clone();
        //get the ip address
        let ip = req.connection_info().remote_addr().unwrap().to_owned();

        let newuser: database::NewUser = database::NewUser{
            email: email,
            pw: pw,
            ip:ip,
            username: uname
        };
        //add the user
        let out: NewUserResponse = insert::add_user(newuser).unwrap();
        
        HttpResponse::Ok().json(out)
    }
}