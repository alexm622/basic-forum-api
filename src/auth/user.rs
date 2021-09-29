pub mod login{
    use crate::structs::responses::post::{LoginResponse};

    pub fn login() -> LoginResponse{
        let response: LoginResponse =LoginResponse {outcome: false,login_token: None, uid: None};

        response
    }
}

pub mod create{
    use crate::structs::database::database::NewUser;
    use crate::structs::auth::auth::Auth;
    use crate::structs::user::user::User;
    use crate::structs::moderation::moderation::ModerationRecord;
    use crate::database::insert::insert;
    use bcrypt::{DEFAULT_COST, hash, verify};
    use std::time::SystemTime;
    use sha256::digest;

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

    pub fn hash_pw(pw: String) -> String{
        let hashed = hash(pw, DEFAULT_COST).unwrap();
        hashed
    }

    pub fn verify_pw(pw: String, hash: String) -> bool{
        let valid = verify(pw, &hash).unwrap();
        valid
    }

    pub fn generate_token(uid: u64) -> String{
        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let magic_number = uid + time;
        let val = digest(magic_number.to_string());
        val
    }
}