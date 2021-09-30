pub mod get{
    use mysql::*;
    use mysql::prelude::*;

    use crate::structs::database::database::IdInfo;
    use crate::database::insert::insert;

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

    pub fn verify_token(token:String, ip:String, uid_test:u64)-> Result<bool>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let mut stmt = conn.prep("SELECT auth_id FROM auth WHERE active_token = :token, ip = :ip")?;
        let aid_vec:Vec<u64> = conn.exec(stmt, params!{
            "token" => token.clone(),
            "ip" => ip.clone(),
        }).expect("Query failed.");
        if aid_vec.len() != 0{
            return Ok(false);
        }
        let aid:u64 = aid_vec[0];
        stmt = conn.prep("SELECT user_id FROM users WHERE auth_id = :aid")?; 
        let uid_vec:Vec<u64> = conn.exec(stmt, params!{
            "aid" => aid,
        }).expect("Query Failed");
        if aid_vec.len() != 0{
            return Ok(false);
        }
        let uid:u64 = uid_vec[0];
        if uid == uid_test{
            return Ok(true);
        }
        Ok(false)
    }
}