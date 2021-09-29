pub mod post{

    use actix_web::{web, HttpResponse};
    
    use crate::structs::requests::post::{MakePost,MakeCat,MakeComment};
    use crate::structs::responses::post::StatusResponse;

    

    pub async fn make_cat(json: web::Json<MakeCat>,req: web::HttpRequest) -> HttpResponse {
            
        log::info!("make category");
        

        let response:StatusResponse;

        HttpResponse::Ok().json("ok")
    }

    pub async fn make_post(json: web::Json<MakePost>,req: web::HttpRequest) -> HttpResponse {
        
        log::info!("make post");
        

        let response:StatusResponse;

        HttpResponse::Ok().json("ok")
    }

    pub async fn make_comment(json: web::Json<MakeComment>,req: web::HttpRequest) -> HttpResponse {
        
        log::info!("make comment");
        

        let response:StatusResponse;

        HttpResponse::Ok().json("ok")
    }
    
}