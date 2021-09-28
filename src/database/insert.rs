pub mod insert{
    use mysql::*;

    use crate::structs::database::database;

    pub const URL:&str = "mysql://server:serverpass@10.40.16.202:3306/forum";

    pub async fn add_user(user: database::NewUser) -> Result<String>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn()?;
        //let stmt = conn.prep("INSERT INTO user (user_id,username,locale,auth_id, moderation_id) VALUES(?,?,?,?,?)")?;
        Ok("test".to_owned())
    }
}