pub mod get{
    use mysql::*;
    use mysql::prelude::*;

    use crate::structs::database::database::IdInfo;
    use crate::database::insert::insert;
    use crate::utils::ip_tools::ip_tools;

    const URL:&str = "mysql://server:serverpass@10.16.40.202:3306/forum";

    pub fn get_id_info(uname: String) -> Result<IdInfo>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let stmt = conn.prep("SELECT user_id, auth_id, moderation_id FROM users WHERE username = :uname")?;
        let res:Vec<IdInfo> = conn.exec_map(stmt, params!{
            "uname" => uname.clone(),
        }, |(user_id, auth_id, moderation_id)|
            IdInfo{
                aid: auth_id,
                uid: user_id,
                mid: moderation_id,
            }).expect("Query failed.");
        if res.len() != 1{
            log::error!("There are more than one entries for username {}", uname);
            let out:IdInfo = IdInfo{
                aid: 0,
                uid: 0,
                mid: 0,
            };
            Ok(out)
        }else{
            Ok(res[0].clone())
        }
        
    }
    pub fn check_token(token:String, aid:u64, ip:String) -> Result<bool>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let stmt = conn.prep("SELECT auth_id FROM auth WHERE active_token = :token")?;
        let res:Vec<u64> = conn.exec(stmt, params!{
            "token" => token.clone(),
        }).expect("Query failed.");
        if res.len() != 0{
            Ok(false)
        }else{
            insert::add_token(token, aid, ip).unwrap();
            Ok(true)
        }
        
    }
    pub fn check_cat(id:u64) -> Result<bool>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let stmt = conn.prep("SELECT cat_id FROM categories WHERE cat_id = :id")?;
        let res:Vec<u64> = conn.exec(stmt, params!{
            "id" => id,
        }).expect("Query failed.");
        if res.len() == 0{
            Ok(false)
        }else{
            Ok(true)
        } 
    }
    pub fn cat_exists(name:String) -> Result<bool>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let stmt = conn.prep("SELECT cat_name FROM categories WHERE cat_name = :name")?;
        let res:Vec<u64> = conn.exec(stmt, params!{
            "name" => name,
        }).expect("Query failed.");
        if res.len() == 0{
            Ok(false)
        }else{
            Ok(true)
        } 
    }
    pub fn check_post(id:u64) -> Result<bool>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let stmt = conn.prep("SELECT post_id FROM categories WHERE post_id = :id")?;
        let res:Vec<u64> = conn.exec(stmt, params!{
            "id" => id,
        }).expect("Query failed.");
        if res.len() == 0{
            Ok(false)
        }else{
            Ok(true)
        } 
    }
    pub fn check_comment(id:String) -> Result<bool>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let stmt = conn.prep("SELECT comment_id FROM categories WHERE comment_id = :id")?;
        let res:Vec<u64> = conn.exec(stmt, params!{
            "comment_id" => id,
        }).expect("Query failed.");
        if res.len() == 0{
            Ok(false)
        }else{
            Ok(true)
        } 
    }

    pub fn  verify_token(token:String, ip:String, uid_test:u64)-> Result<bool>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        log::info!("testing ip {} with token {}", ip.clone(), token.clone());
        let mut stmt = conn.prep("SELECT auth_id FROM auth WHERE active_token = :token AND token_ip = :ip")?;
        let aid_vec:Vec<u64> = conn.exec(stmt, params!{
            "token" => token.clone(),
            "ip" => ip_tools::strip_port(ip.clone()),
        }).expect("Query failed.");
        if aid_vec.len() != 1{
            log::info!("Token failed for ip {}", ip.clone());
            return Ok(false);
        }
        let aid:u64 = aid_vec[0];
        log::info!("testing aid {}", aid);
        stmt = conn.prep("SELECT user_id FROM users WHERE auth_id = :aid")?; 
        let uid_vec:Vec<u64> = conn.exec(stmt, params!{
            "aid" => aid,
        }).expect("Query Failed");
        if aid_vec.len() != 1{
            log::info!("UID failed for ip {}", ip.clone());
            return Ok(false);
        }
        let uid:u64 = uid_vec[0];
        if uid == uid_test{
            log::info!("UID and Token passed for ip {}", ip.clone());
            return Ok(true);
        }
        Ok(false)
    }
}