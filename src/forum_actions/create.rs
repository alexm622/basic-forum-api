pub mod create{
    use crate::structs::responses::post::StatusResponse;
    use crate::structs::requests::post::{MakeCat, MakeComment,MakePost};
    use crate::database::insert::insert;
    use crate::database::get::get;
    pub fn create_post(req: MakePost, ip:String)-> StatusResponse{
        let mut res:StatusResponse = StatusResponse{
            response_code: 1,
            redirect: None,
        };
        let good_token:bool = get::verify_token(req.user_token.clone(), ip, req.user).unwrap();
        if good_token{
            let post_id = insert::create_post(req).unwrap();
        res.redirect = Some(post_id.to_string().to_owned());
        }else{
            res.redirect = Some("bad token+ip combo".to_owned());
        }
        
        return res;
    }
    pub fn create_category(req: MakeCat, ip:String)-> StatusResponse{
        let mut res:StatusResponse = StatusResponse{
            response_code: 1,
            redirect: None,
        };
        let good_token:bool = get::verify_token(req.user_token.clone(), ip, req.user).unwrap();
        let exists:bool = get::cat_exists(req.name.clone()).unwrap();
        if good_token & !exists{
            let cat_id:u64 = insert::create_cat(req).unwrap();
            res.redirect = Some(cat_id.to_string().to_owned());
        }else{
            if good_token {
                res.redirect = Some("exists".to_owned());
            }else{
                res.redirect = Some("bad token+ip combo".to_owned());
            }
        }
        
        return res;
    }

    pub fn create_comment(req: MakeComment, ip:String) -> StatusResponse{
        let mut res:StatusResponse = StatusResponse{
            response_code: 1,
            redirect: None,
        };
        
        let good_token:bool = get::verify_token(req.user_token.clone(), ip, req.user).unwrap();
        if good_token{
            let comment_id:u64 = insert::create_comment(req).unwrap();
            res.redirect = Some(comment_id.to_string().to_owned());
        }else{
            res.redirect = Some("bad token+ip combo".to_owned());
        }
        return res;
    }
}