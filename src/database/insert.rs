pub mod insert{
    use mysql::*;
    use mysql::prelude::*;

    use crate::structs::database::database;
    use crate::structs::auth::auth::Auth;
    use crate::structs::moderation::moderation::ModerationRecord;
    use crate::structs::user::user::User;
    use crate::auth::user::create;
    use crate::structs::requests::post::{MakeCat, MakeComment, MakePost};
    use crate::structs::responses::post::NewUserResponse;
    use crate::utils::ip_tools::ip_tools;


    use std::time::SystemTime;

    const URL:&str = "mysql://server:serverpass@10.16.40.202:3306/forum";

    //create a new user
    pub fn add_user(newuser: database::NewUser) -> Result<NewUserResponse>{//return userid
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        //create the auth entry for user
        let auth: Auth = create::create_auth(newuser.clone());

        //create the mod record for user
        let modrec: ModerationRecord = create::create_mod_rec();

        //create the user object
        let user: User = create::create_user(newuser.clone(), auth.auth_id, modrec.moderation_id);

        //prepare the sql statement
        let stmt = conn.prep("INSERT INTO users (username,locale,auth_id, moderation_id) VALUES(:uname,:locale,:auth_id,:mod_id)")?;
        //execute the statement
        conn.exec_drop(&stmt, params!{
            "uname" => user.username.clone(),
            "locale" => user.locale.clone(),
            "auth_id" => user.auth_id,
            "mod_id" => user.moderation_id,
        },).unwrap();
        let mut newtoken:bool;
        let mut token:String;
        let uid:u64 = conn.last_insert_id();
        while{
            //generate a new token
            token = create::generate_token(uid);
            //check to see if the token is not a duplicate
            newtoken = crate::database::get::get::check_token(token.clone(), user.auth_id, newuser.ip.clone()).unwrap();
            //test to see if the token is original
            newtoken != true
        }{}
        let resp:NewUserResponse = NewUserResponse{response_code:100, outcome: conn.last_insert_id() > 0,token: Some(token), uid: Some(uid)};

        //return the userid
        Ok(resp)
    }
    //insert auth account for user
    pub fn add_auth(new_auth: &Auth) -> Result<u64>{
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        //prepare sql statement
        let stmt = conn.prep("INSERT INTO auth (creation_ip, last_ip, email, pwhash, uname, last_change, created) VALUES(:creation_ip, :last_ip, :email, :pwhash, :uname, :last_change, :created)")?;
        //execute statement
        conn.exec_drop(&stmt, params!{
            "creation_ip" => ip_tools::strip_port(new_auth.creation_ip.clone()),
            "last_ip" => ip_tools::strip_port(new_auth.last_ip.clone()),
            "email" => new_auth.email.clone(),
            "pwhash" => new_auth.hash.clone(),
            "uname" => new_auth.uname.clone(),
            "last_change" => new_auth.last_change,
            "created" => new_auth.created,
        },).unwrap();

        //return the auth id
        Ok(conn.last_insert_id())
    }

    //create a moderation record for user
    pub fn add_modrec() -> Result<u64>{
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        //prepare sql statement
        let stmt = conn.prep("INSERT INTO user_moderation (infraction_counter) VALUES(:infraction_counter)")?;

        //execute statement
        conn.exec_drop(&stmt, params!{
            "infraction_counter" => "0",
        },).unwrap();
        
        //return the moderation id
        Ok(conn.last_insert_id())
    }

    //insert a new token into the database for user with aid u64
    pub fn add_token(token: String, aid: u64, ip: String) -> Result<bool>{
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        //get the current date
        let date = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        //add the token for aid
        log::info!("adding token {} to auth for aid {}", token, aid);
        let stmt = conn.prep("UPDATE auth SET active_token = :token, token_ip = :ip, token_date = :date WHERE auth_id = :aid")?;

        //execute statement
        conn.exec_drop(&stmt, params!{
            "token" => token,
            "aid" => aid,
            "ip" => ip_tools::strip_port(ip),
            "date" => date,
        },).unwrap();
        //return true
        Ok(true)
    }
    //create a new category
    pub fn create_cat(cat: MakeCat) -> Result<u64>{
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        //get the current date
        let date = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        log::info!("adding cat {} to categories for uid {}", cat.name.clone(), cat.user);
        //prepare the sql statement
        let stmt = conn.prep("INSERT INTO categories (creator_id, cat_name, cat_desc, creation_date) VALUES(:uid, :name, :desc, :date)")?;
        //execute statement
        conn.exec_drop(&stmt, params!{
            "date" => date,
            "desc" => cat.desc.clone(),
            "uid" => cat.user,
            "name" => cat.name.clone(),
        },).unwrap();
        //get cat id
        let cat_id:u64 = conn.last_insert_id();
        //return the category id
        Ok(cat_id)
    }
    //create a new post
    pub fn create_post(post: MakePost) -> Result<u64>{
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        //get the current date
        let date = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        log::info!("adding post {} to category {} for uid {}", post.name.clone(), post.cat, post.user);
        //prepare statement
        let stmt = conn.prep("INSERT INTO posts (creator_id, cat_id, content, name, creation_date) VALUES(:uid, :cat_id, :content, :name, :date)")?;
        //execute statement
        conn.exec_drop(&stmt, params!{
            "cat_id" => post.cat,
            "date" => date,
            "content" => post.contents.clone(),
            "uid" => post.user,
            "name" => post.name.clone(),
        },).unwrap();
        //get the post id
        let post_id:u64 = conn.last_insert_id();
        //return the post id
        Ok(post_id)
    }
    //create a new comment
    pub fn create_comment(comment: MakeComment) -> Result<u64>{
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        //get the current date
        let date = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        log::info!("adding comment to post {} for uid {}", comment.post, comment.user);
        //prepare sql statement
        let stmt = conn.prep("INSERT INTO comments (creator_id, post_id, parent_id, content, creation_date) VALUES(:uid, :post_id, :parent_id, :content, :date)")?;
        //execute sql statement
        conn.exec_drop(&stmt, params!{
            "post_id" => comment.post,
            "date" => date,
            "content" => comment.contents.clone(),
            "uid" => comment.user,
            "parent_id" => comment.parent,
        },).unwrap();
        //get comment id
        let comment_id:u64 = conn.last_insert_id();
        //return comment id
        Ok(comment_id)
    }
}