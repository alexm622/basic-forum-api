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
}