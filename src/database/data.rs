pub mod data{
    use mysql::*;
    use mysql::prelude::*;
    use crate::structs::database::database::*;
    //the mysql server url
    const URL:&str = "mysql://server:serverpass@10.16.40.202:3306/forum";
    //get categories
    //select cat_id,cat_name,cat_desc from forum.categories WHERE is_archived = 0 AND is_deleted = 0 ORDER BY creation_date DESC LIMIT 10 OFFSET 0;
    //get post
    //select post_id,name,content,creation_date from forum.posts WHERE is_archived = 0 AND is_deleted = 0 AND cat_id = 1 ORDER BY creation_date DESC LIMIT 10 OFFSET 0;
    //get comment
    //select comment_id,content,creation_date from forum.comments WHERE is_archived = 0 AND is_deleted = 0 AND post_id = 1 AND cat_id = 1 AND parent_id = 0 ORDER BY creation_date DESC LIMIT 10 OFFSET 0;
    //get the id info for a username
    pub fn get_cats(offset: u64) -> Option<Vec<CategoryInfo>>{
        //create the connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        //create the statement
        let stmt = conn.prep("select cat_id,cat_name,cat_desc from forum.categories WHERE is_archived = 0 AND is_deleted = 0 ORDER BY creation_date DESC LIMIT 10 OFFSET :offset").unwrap();

        //query the server
        let res:Vec<CategoryInfo> = conn.exec_map(stmt, params!{
            "offset" => offset,
        }, |(cat_id, cat_name, cat_desc)|
            CategoryInfo{
                cat_desc: cat_desc,
                cat_name: cat_name,
                cat_id: cat_id,
            }).expect("Query failed.");
        if res.len() == 0{
            return None;
        }
        return Some(res);
        
    }
    pub fn get_posts(cat_id:u64, offset: u64) -> Option<Vec<PostInfo>>{
        //create the connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        //create the statement
        let stmt = conn.prep("select post_id,name,content,creation_date,creator_id from forum.posts WHERE is_archived = 0 AND is_deleted = 0 AND cat_id = :cat_id ORDER BY creation_date DESC LIMIT 5 OFFSET :offset;").unwrap();

        //query the server
        let res:Vec<PostInfo> = conn.exec_map(stmt, params!{
            "offset" => offset,
            "cat_id" => cat_id,
        }, |(post_id, name, content, creator_id, creation_date)|
            PostInfo{
                post_id: post_id,
                name: name,
                content: content,
                creator_id: creator_id,
                creation_date: creation_date
            }).expect("Query failed.");
        if res.len() == 0{
            return None;
        }
        return Some(res);
        
    }
    pub fn get_post(post_id: u64) -> Option<PostInfo>{
        //create the connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        //create the statement
        let stmt = conn.prep("select post_id,name,content,creation_date,creator_id from forum.posts WHERE post_id = :post_id").unwrap();

        //query the server
        let res:Vec<PostInfo> = conn.exec_map(stmt, params!{
            "post_id" => post_id,
        }, |(post_id, name, content, creator_id, creation_date)|
            PostInfo{
                post_id: post_id,
                name: name,
                content: content,
                creator_id: creator_id,
                creation_date: creation_date
            }).expect("Query failed.");
        if res.len() == 0{
            return None;
        }
        return Some(res[0].clone());
        
    }

    pub fn get_comments(cat_id:u64, post_id:u64, parent_id:u64, offset: u64) -> Option<Vec<CommentInfo>>{
        //create the connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        //create the statement
        let stmt = conn.prep("select comment_id,content,creator_id,creation_date from forum.comments WHERE is_archived = 0 AND is_deleted = 0 AND post_id = :post_id AND cat_id = :cat_id AND parent_id = :parent_id ORDER BY creation_date DESC LIMIT 10 OFFSET :offset").unwrap();

        //query the server
        let res:Vec<CommentInfo> = conn.exec_map(stmt, params!{
            "cat_id" => cat_id,
            "post_id" => post_id,
            "parent_id" => parent_id,
            "offset" => offset,
        }, |(comment_id, content, creator_id, creation_date)|
        CommentInfo{
                comment_id: comment_id,
                content:content,
                creator_id: creator_id,
                creation_date:creation_date,
            }).expect("Query failed.");
        if res.len() == 0{
            return None;
        }
        return Some(res);
        
    }

    
}