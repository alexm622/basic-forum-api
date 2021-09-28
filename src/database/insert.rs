pub mod insert{
    use mysql::*;
    use mysql::prelude::*;

    use crate::structs::database::database;
    use crate::structs::auth::auth::Auth;
    use crate::structs::moderation::moderation::ModerationRecord;
    use crate::structs::user::user::User;
    use crate::auth::user::create;

    pub const URL:&str = "mysql://server:serverpass@10.40.16.202:3306/forum";

    pub fn add_user(newuser: database::NewUser) -> Result<String>{//idk what this should be returning realistically
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let auth: Auth = create::createAuth(newuser.clone());
        let modrec: ModerationRecord = create::createModRec(newuser.clone());
        let user: User = create::createUser(newuser.clone(), auth.auth_id, modrec.moderation_id);
        let stmt = conn.prep("INSERT INTO user (username,locale,auth_id, moderation_id) VALUES(:uname,:locale,:auth_id,:mod_id)")?;
        conn.exec_drop(&stmt, params!{
            "uname" => user.username,
            "locale" => user.locale,
            "auth_id" => user.auth_id,
            "mod_id" => user.moderation_id,
        },).unwrap();

        Ok("test".to_owned())
    }
    pub fn add_auth(newuser: database::NewUser) -> Result<String>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let stmt = conn.prep("INSERT INTO user (username,locale,auth_id, moderation_id) VALUES(:uname,:locale,:auth_id,:mod_id)")?;
        /*conn.exec_drop(&stmt, params!{
            
        },).unwrap();*/
        //do something with this

        Ok("test".to_owned())
    }
    pub fn add_modrec(newuser: database::NewUser) -> Result<String>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let stmt = conn.prep("INSERT INTO user (username,locale,auth_id, moderation_id) VALUES(:uname,:locale,:auth_id,:mod_id)")?;
        /*conn.exec_drop(&stmt, params!{
            
        },).unwrap();*/
        //do something with this

        Ok("test".to_owned())
    }
}