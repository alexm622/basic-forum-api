pub mod login{
    use crate::structs::responses::post::{LoginResponse,NewUser};

    pub fn login() -> LoginResponse{
        let mut response: LoginResponse =LoginResponse {outcome: false,login_token: None, uid: None};

        response
    }
    pub fn createUser() -> NewUser{
        let mut response: NewUser = NewUser {response_code: 0, outcome: false, uid: None, token: None};

        response

    }
}

pub mod create{
    use crate::structs::database::database::NewUser;
    use crate::structs::auth::auth::Auth;
    use crate::structs::user::user::User;
    use crate::structs::moderation::moderation::ModerationRecord;

    pub fn createAuth(newuser: NewUser) -> Auth{
        let mut auth: Auth = Auth{
            auth_id: 0, //this is generated automatically by mysql
            creation_ip: newuser.ip.clone(),
            last_ip: newuser.ip,
            email: newuser.email,
            hash: "".to_owned(),
            uname: newuser.username,
            last_change: 0,
            created: 0,
            active_token: None,
            token_ip: None,
            token_date: None
        };
        auth
    }

    pub fn createModRec(newuser: NewUser) -> ModerationRecord{
        let mut modrec: ModerationRecord = ModerationRecord{
            moderation_id:0, //something to generate this
            bans:None,
            mutes:None,
            global_info:None,
            infraction_counter:0,
            global_infractions:0,
        };
        modrec

    }

    pub fn createUser(newuser: NewUser, aid: u64, mid: u64) -> User{
        let mut user: User = User{
            id: 0, //make something to get this
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
}