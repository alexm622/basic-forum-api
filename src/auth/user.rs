pub mod login{
    use crate::structs::responses::post::{LoginResponse};
    use crate::structs::requests::post::Login;
    use crate::auth::user::create;
    use crate::database::get;
    use crate::structs::database::IdInfo;
    
    //handle a login post request
    pub fn login(request: Login, ip:String) -> LoginResponse{
        //create an empty login response
        let mut response: LoginResponse =LoginResponse {outcome: false,login_token: None, uid: None};
        
        //get the id info for said username
        let id:IdInfo = get::get_id_info(request.uname).unwrap();
        //if the auth id is zero (not exist) then the login failed
        if id.aid == 0{
            return response;
        }
        
        //this doesnt verify password?
        let hash:String = get::get_pw_hash(id.aid).unwrap();
        let good_pw:bool = create::verify_pw(request.pw.clone(), hash);
        if !good_pw{
            return response;
        }
        
        //token and test to see if token does not exist already
        let mut newtoken:bool;
        let mut token:String;
        //do while loop
        while{
            //generate a new token
            token = create::generate_token(id.uid);
            //check to see if the token is not a duplicate
            newtoken = get::check_token(token.clone(), id.aid, ip.clone()).unwrap();
            //test to see if the token is original
            newtoken != true
        }{}
        //respond with the proper information
        response.login_token = Some(token.clone());
        response.outcome = true;
        response.uid = Some(id.uid);
        response
    }
}

pub mod create{
    
    use crate::structs::database::NewUser;
    use crate::structs::auth::Auth;
    use crate::structs::user::User;
    use crate::structs::moderation::ModerationRecord;
    use crate::database::insert;
    use bcrypt::{DEFAULT_COST, hash, verify};
    use std::time::SystemTime;
    use sha256::digest;
    //create an auth object
    pub fn create_auth(newuser: NewUser) -> Auth{
        let mut auth: Auth = Auth{
            auth_id: 0, //this is generated automatically by mysql
            creation_ip: newuser.ip.clone(),
            last_ip: newuser.ip,
            email: newuser.email,
            hash: hash_pw(newuser.pw),
            uname: newuser.username,
            last_change: 0,
            created: 0,
            active_token: None,
            token_ip: None,
            token_date: None
        };

        auth.auth_id = insert::add_auth(&auth).unwrap();
        auth
    }
    //create a mod record object
    pub fn create_mod_rec() -> ModerationRecord{
        let mut modrec: ModerationRecord = ModerationRecord{
            moderation_id:0, //this is generated automatically by mysql
            bans:None,
            mutes:None,
            global_info:None,
            infraction_counter:0,
            global_infractions:0,
        };
        modrec.moderation_id = insert::add_modrec().unwrap();
        modrec

    }
    //create a basic user object
    pub fn create_user(newuser: NewUser, aid: u64, mid: u64) -> User{
        let user: User = User{
            id: 0, //this is generated automatically by mysql
            username:newuser.username,
            avatar: None,
            bot:false,
            mfa_enabled:false,
            locale:"EN".to_owned(),
            flags:0,
            premium_type:0,
            public_flags:0,
            auth_id:aid,
            moderation_id:mid,
        };
        user
    }

    //hash the password
    pub fn hash_pw(pw: String) -> String{
        let hashed = hash(pw, DEFAULT_COST).unwrap();
        hashed
    }

    //check the password against the hash
    pub fn verify_pw(pw: String, hash: String) -> bool{
        let valid = verify(pw, &hash).unwrap();
        valid
    }

    //create a new user token and return it
    pub fn generate_token(uid: u64) -> String{
        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let magic_number = uid + time;
        let val = digest(magic_number.to_string());
        val
    }
}