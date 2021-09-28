pub mod insert{
    use mysql::*;
    use mysql::prelude::*;

    use crate::structs::database::database;
    use crate::structs::auth::auth::Auth;
    use crate::structs::moderation::moderation::ModerationRecord;
    use crate::structs::user::user::User;
    use crate::auth::user::create;

    pub const URL:&str = "mysql://server:serverpass@10.40.16.202:3306/forum";

    pub async fn add_user(newuser: database::NewUser) -> Result<String>{
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
}