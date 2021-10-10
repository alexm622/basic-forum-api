pub mod login{
    use actix_web::{web, HttpResponse};
    use crate::structs::responses::get::Exists;
    use crate::structs::requests::get::{UnameExistsCheck, EmailExistsCheck};
    use crate::database::get;
    
    //handle a username check request
    pub async fn check_uname(path: web::Path<UnameExistsCheck>) -> HttpResponse {
        log::info!("uname check handler");
        //get the remote ip address
        let exists:bool = get::check_uname(path.uname.to_owned()).unwrap();
        let response:Exists = Exists{
            exists: exists,
        };

        HttpResponse::Ok().json(response)
    }
    //handle an email check request
    pub async fn check_email(path: web::Path<EmailExistsCheck>) -> HttpResponse {
        log::info!("email check handler");
        //get the remote ip address
        let exists:bool = get::check_email(path.email.to_owned()).unwrap();
        let response:Exists = Exists{
            exists: exists,
        };
                
        HttpResponse::Ok().json(response)
    }
}

    