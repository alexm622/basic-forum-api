pub mod data{
    use actix_web::{web, HttpResponse};
    use crate::structs::responses::get::{Categories,Posts,Comments};
    use crate::structs::requests::get::{GetCategories,GetComments,GetPosts, GetCommentsNoParent};
    use crate::structs::database::database::{CategoryInfo,PostInfo,CommentInfo};
    use crate::database::data::data as sql_data;
    //get the categories
    pub async fn get_categories(path: web::Path<GetCategories>) -> HttpResponse {
        log::info!("getting categories");
        let resp_vec:Option<Vec<CategoryInfo>> = sql_data::get_cats(path.offset);
        let mut response:Categories = Categories{
            has_results: false,
            num_results: None,
            results: resp_vec.clone(),
        };
        if resp_vec.is_some(){
            let results:Vec<CategoryInfo> = resp_vec.unwrap();
            response.has_results = true;
            response.num_results = Some(results.len());
        }

        HttpResponse::Ok().json(response)
    }
    
    //handle a username check request
    pub async fn get_posts(path: web::Path<GetPosts>) -> HttpResponse {
        log::info!("getting posts");
        let resp_vec:Option<Vec<PostInfo>> = sql_data::get_posts(path.cat_id, path.offset);
        let mut response:Posts = Posts{
            has_results: false,
            num_results: None,
            results: resp_vec.clone(),
        };
        if resp_vec.is_some(){
            let results:Vec<PostInfo> = resp_vec.unwrap();
            response.has_results = true;
            response.num_results = Some(results.len());
        }

        HttpResponse::Ok().json(response)
    }
    //handle a username check request
    pub async fn get_comments(path: web::Path<GetComments>) -> HttpResponse {
        log::info!("getting comments");
        let parent_id:u64;
        if path.parent_id.is_none(){
            parent_id = 0;
        }else{
            parent_id = path.parent_id.unwrap();
        }
        let resp_vec:Option<Vec<CommentInfo>> = sql_data::get_comments(path.cat_id, path.post_id,parent_id , path.offset);
        let mut response:Comments = Comments{
            has_results: false,
            num_results: None,
            results: resp_vec.clone(),
        };
        if resp_vec.is_some(){
            let results:Vec<CommentInfo> = resp_vec.unwrap();
            response.has_results = true;
            response.num_results = Some(results.len());
        }

        HttpResponse::Ok().json(response)
    }

    //handle a username check request
    pub async fn get_comments_no_parent(path: web::Path<GetCommentsNoParent>) -> HttpResponse {
        log::info!("getting comments");
        
        let resp_vec:Option<Vec<CommentInfo>> = sql_data::get_comments(path.cat_id, path.post_id,0 , path.offset);
        let mut response:Comments = Comments{
            has_results: false,
            num_results: None,
            results: resp_vec.clone(),
        };
        if resp_vec.is_some(){
            let results:Vec<CommentInfo> = resp_vec.unwrap();
            response.has_results = true;
            response.num_results = Some(results.len());
        }

        HttpResponse::Ok().json(response)
    }
}