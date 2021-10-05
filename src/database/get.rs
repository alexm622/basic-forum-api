pub mod get{
    use mysql::*;
    use mysql::prelude::*;

    use crate::structs::database::database::IdInfo;
    use crate::database::insert::insert;
    use crate::utils::ip_tools::ip_tools;

    //the mysql server url
    const URL:&str = "mysql://server:serverpass@10.16.40.202:3306/forum";

    //get the id info for a username
    pub fn get_id_info(uname: String) -> Result<IdInfo>{
        //create the connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        //create the statement
        let stmt = conn.prep("SELECT user_id, auth_id, moderation_id FROM users WHERE username = :uname")?;

        //query the server
        let res:Vec<IdInfo> = conn.exec_map(stmt, params!{
            "uname" => uname.clone(),
        }, |(user_id, auth_id, moderation_id)|
            IdInfo{
                aid: auth_id,
                uid: user_id,
                mid: moderation_id,
            }).expect("Query failed.");
        //if there is more than one response or there is 0 responses return a default
        if res.len() != 1{
            log::error!("There are more than one entries for username {} or that username does not exist", uname);
            //create the output struct
            let out:IdInfo = IdInfo{
                aid: 0,
                uid: 0,
                mid: 0,
            };
            //return the struct
            Ok(out)
        }else{
            //return the only result
            Ok(res[0].clone())
        }
        
    }
    //check to see if token is not taken/currently in use
    // if not initialize said token
    pub fn check_token(token:String, aid:u64, ip:String) -> Result<bool>{
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        //prepare the sql query
        let stmt = conn.prep("SELECT auth_id FROM auth WHERE active_token = :token")?;

        //test to see if token exists
        let res:Vec<u64> = conn.exec(stmt, params!{
            "token" => token.clone(),
        }).expect("Query failed.");
        //if token already exists then try for a new token
        if res.len() != 0{
            Ok(false)
        }else{
            //token does not exist so initialize it
            insert::add_token(token, aid, ip).unwrap();
            Ok(true)
        }
        
    }

    //check to see if category exists
    pub fn check_cat(id:u64) -> Result<bool>{
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        //prepare sql query
        let stmt = conn.prep("SELECT cat_id FROM categories WHERE cat_id = :id")?;

        //query database to see if category id exists
        let res:Vec<u64> = conn.exec(stmt, params!{
            "id" => id,
        }).expect("Query failed.");
        //if it does not exist return false
        //if it does exist return true
        if res.len() == 0{
            Ok(false)
        }else{
            Ok(true)
        } 
    }
    //check to see if a category exists
    pub fn cat_exists(name:String) -> Result<bool>{
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        //prepare sql query
        let stmt = conn.prep("SELECT cat_name FROM categories WHERE cat_name = :name")?;

        //query the server
        let res:Vec<u64> = conn.exec(stmt, params!{
            "name" => name,
        }).expect("Query failed.");

        //if the name does not exist return false
        if res.len() == 0{
            Ok(false)
        }else{
            //the category exists so we're returning true
            Ok(true)
        } 
    }
    //check to see if post exists
    pub fn check_post(id:u64) -> Result<bool>{
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        //prepare the sql query
        let stmt = conn.prep("SELECT post_id FROM posts WHERE post_id = :id")?;
        let res:Vec<u64> = conn.exec(stmt, params!{
            "id" => id,
        }).expect("Query failed.");

        //if post id does not exist return false, but if it exists then return true
        if res.len() == 0{
            Ok(false)
        }else{
            Ok(true)
        } 
    }

    //check to see if comment with id exists
    pub fn check_comment(id:u64) -> Result<bool>{
        //if the parent comment does not exist then it is the first comment
        if id == 0{
            return Ok(true);
        }
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        
        //prepare SQL query
        let stmt = conn.prep("SELECT comment_id FROM comments WHERE comment_id = :id")?;
        let res:Vec<u64> = conn.exec(stmt, params!{
            "id" => id,
        }).expect("Query failed.");

        //if the parent does not exist return false if it does exist return true
        if res.len() == 0{
            Ok(false)
        }else{
            Ok(true)
        } 
    }
    //check to see if uname exists
    pub fn check_uname(uname:String) -> Result<bool>{
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        //prepare the sql query
        let stmt = conn.prep("SELECT uname FROM auth WHERE uname = :uname")?;
        let res:Vec<u64> = conn.exec(stmt, params!{
            "uname" => uname,
        }).expect("Query failed.");

        //if post id does not exist return false, but if it exists then return true
        if res.len() == 0{
            Ok(true)
        }else{
            Ok(false)
        } 
    }
    //check to see if email exists
    pub fn check_email(email:String) -> Result<bool>{
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        //prepare the sql query
        let stmt = conn.prep("SELECT uname FROM auth WHERE email = :email")?;
        let res:Vec<u64> = conn.exec(stmt, params!{
            "email" => email,
        }).expect("Query failed.");

        //if post id does not exist return false, but if it exists then return true
        if res.len() == 0{
            Ok(true)
        }else{
            Ok(false)
        } 
    }


    //check to see if comment with id exists
    pub fn get_pw_hash(aid:u64) -> Result<String>{
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        
        //prepare SQL query
        let stmt = conn.prep("SELECT pwhash FROM auth WHERE auth_id = :aid")?;
        let res:Vec<String> = conn.exec(stmt, params!{
            "aid" => aid,
        }).expect("Query failed.");

        //if the authid does not exist return empty
        if res.len() == 0{
            return Ok("".to_owned());
        }else{
            return Ok(res[0].clone());
        } 
    }

    //verify a token for given ip address and uid
    pub fn  verify_token(token:String, ip:String, uid_test:u64)-> Result<bool>{
        //initialize connection
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        log::info!("testing ip {} with token {}", ip.clone(), token.clone());

        //prepare sql query
        let mut stmt = conn.prep("SELECT auth_id FROM auth WHERE active_token = :token AND token_ip = :ip")?;

        //execute query
        let aid_vec:Vec<u64> = conn.exec(stmt, params!{
            "token" => token.clone(),
            "ip" => ip_tools::strip_port(ip.clone()),
        }).expect("Query failed.");

        //if there are no entries for this ip token combo then this is a bad token
        if aid_vec.len() != 1{
            log::info!("Token failed for ip {}", ip.clone());
            return Ok(false);
        }
        //get the auth id of the token ip combo
        let aid:u64 = aid_vec[0];
        log::info!("testing aid {}", aid);

        //prepare sql query
        stmt = conn.prep("SELECT user_id FROM users WHERE auth_id = :aid")?; 

        //execute the statement
        let uid_vec:Vec<u64> = conn.exec(stmt, params!{
            "aid" => aid,
        }).expect("Query Failed");

        //if there are no users with that aid then return false
        if aid_vec.len() != 1{
            log::info!("UID failed for ip {}", ip.clone());
            return Ok(false);
        }
        //get the uid with the matching auth id
        let uid:u64 = uid_vec[0];

        //if the given uid and the uid on the server match then it passes the test
        if uid == uid_test{
            log::info!("UID and Token passed for ip {}", ip.clone());
            return Ok(true);
        }

        //return false if above does not pass
        Ok(false)
    }
}