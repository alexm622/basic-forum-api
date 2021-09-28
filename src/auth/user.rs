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