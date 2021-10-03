pub mod post{

    use actix_web::{web, HttpResponse};
    
    use crate::structs::requests::post::{MakePost,MakeCat,MakeComment};
    use crate::structs::responses::post::StatusResponse;
    use crate::forum_actions::create::create;
    
    //category creation handler
    pub async fn make_cat(json: web::Json<MakeCat>,req: web::HttpRequest) -> HttpResponse {
        
        log::info!("make category");
        let newcat:MakeCat = json.into_inner();

        //get the user's ip
        let ip = req.connection_info().remote_addr().unwrap().to_owned();
        
        //try to create the category
        let response:StatusResponse = create::create_category(newcat, ip);

        HttpResponse::Ok().json(response)
    }

    //make a post
    pub async fn make_post(json: web::Json<MakePost>,req: web::HttpRequest) -> HttpResponse {
        
        log::info!("make post");
        let newpost:MakePost = json.into_inner();
        //get the remote ip
        let ip = req.connection_info().remote_addr().unwrap().to_owned();
        //try to create a post
        let response:StatusResponse = create::create_post(newpost, ip);

        HttpResponse::Ok().json(response)
    }

    //make a comment
    pub async fn make_comment(json: web::Json<MakeComment>,req: web::HttpRequest) -> HttpResponse {
        
        log::info!("make comment");
        let newcomment:MakeComment = json.into_inner();
        //get the remote ip
        let ip = req.connection_info().remote_addr().unwrap().to_owned();
        //try to make the comment
        let response:StatusResponse = create::create_comment(newcomment, ip);

        HttpResponse::Ok().json(response)
    }
    
}