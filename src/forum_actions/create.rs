pub mod create{
    use crate::structs::responses::post::StatusResponse;
    use crate::structs::requests::post::{MakeCat, MakeComment,MakePost};
    use crate::database::insert;
    use crate::database::get;
    use crate::forum_actions::cleaner;
    use ammonia::clean;

    //create a new category
    pub fn create_category(mut req: MakeCat, ip:String)-> StatusResponse{
        //create the status response
        let mut res:StatusResponse = StatusResponse{
            response_code: 1,
            redirect: None,
        };
        //check to see if the token is valid
        let good_token:bool = get::verify_token(req.user_token.clone(), ip, req.user).unwrap();
        //see if category exists
        let exists:bool = get::cat_exists(req.name.clone()).unwrap();
        //if the token is good and the category does not exist create the category
        if good_token & !exists{
            req.desc = clean(&req.desc);
            req.name = cleaner::clean_full(req.name);
            let cat_id:u64 = insert::create_cat(req).unwrap();
            //set the status to have the cat id
            res.redirect = Some(cat_id.to_string().to_owned());
        }else{
            if good_token {
                //the token is good, but category exists
                res.redirect = Some("exists".to_owned());
            }else{
                res.redirect = Some("bad token+ip combo".to_owned());
            }
        }
        //return the new response
        return res;
    }
    
    //create a post
    pub fn create_post(mut req: MakePost, ip:String)-> StatusResponse{
        //create the status response
        let mut res:StatusResponse = StatusResponse{
            response_code: 1,
            redirect: None,
        };
        //check to see if token is valid
        let good_token:bool = get::verify_token(req.user_token.clone(), ip, req.user).unwrap();
        //check to see if category is valid
        let good_parent:bool = get::check_cat(req.cat).unwrap();

        //if all tests pass create the post
        if good_token & good_parent{
            req.contents = cleaner::clean_full(req.contents);
            req.name = clean(&req.name);
            
            let post_id = insert::create_post(req).unwrap();
            res.redirect = Some(post_id.to_string().to_owned());
        }else{
            //if the token is good then the category does not exist
            if good_token {
                res.redirect = Some("category does not exist".to_owned());
            }else{
                res.redirect = Some("bad token+ip combo".to_owned());
            }
        }
        //return the http response
        return res;
    }
    //create a new comment
    pub fn create_comment(mut req: MakeComment, ip:String) -> StatusResponse{
        let mut res:StatusResponse = StatusResponse{
            response_code: 1,
            redirect: None,
        };
        //check if token is valid
        let good_token:bool = get::verify_token(req.user_token.clone(), ip, req.user).unwrap();
        //check to see if post exists
        let good_post:bool = get::check_post(req.post).unwrap();
        //check to see if the parent comment is good
        let good_parent:bool = get::check_comment(req.parent).unwrap();

        //if all tests pass then create comment
        if good_token & good_parent & good_post{
            req.contents = clean(&req.contents);
            let comment_id:u64 = insert::create_comment(req).unwrap();
            //return the comment id in status
            res.redirect = Some(comment_id.to_string().to_owned());
        }else{
            //if the token is good then figure out what failed
            if good_token {
                //if the post is good then the parent comment is bad
                if good_post{
                    res.redirect = Some("bad parent".to_owned());
                }else{
                    //post is bad
                    res.redirect = Some("bad post".to_owned());
                }
            }else{
                //token + ip is bad
                res.redirect = Some("bad token+ip combo".to_owned());
            }
        }
        //return the new response
        return res;
    }
}