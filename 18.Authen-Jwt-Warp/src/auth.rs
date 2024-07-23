use core::fmt;
use jsonwebtoken::{Header,Algorithm,encode,EncodingKey};
use chrono::{prelude::*, Duration};
use serde::Serialize;
use crate::error::Error;

// 4.2.1 Set the Role modle

#[derive(Clone, PartialEq)]
pub enum Role{
    User,
    Admin,
}

impl Role{
    pub fn from_str(role: &str)-> Self{
        match role {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

impl fmt::Display for Role{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Role::Admin => write!(f,"Admin"),
            Role::User => write!(f,"Admin"),
        }
    }
}

// 4.2.3.1 Set Paypload modle
#[derive(Serialize)]
struct Paypload{
    sub: String,
    role: String,
    exp: usize,
}


// 4.2.3 Fnc: create_jwt
pub fn create_jwt(uid: &str, role: &Role) -> Result<String, Error>{

    // 4.2.3.1 Header
    let headers = Header::new(Algorithm::HS256);

    // 4.2.3.3 Paypload
    let expr = Utc::now()
        .checked_add_signed(Duration::seconds(60))
        .expect("Valid timestamp")
        .timestamp();
    
    let paypload = Paypload{
        sub: uid.to_owned(),
        role: role.to_string(),
        exp: expr as usize,
    };

    // 4.2.3.4 encode (include signature)
    const JWT_SECRETE: &[u8] = b"secret";
    encode(&headers, &paypload, &EncodingKey::from_secret(JWT_SECRETE))
        .map_err(|_| Error::JWTCREATEERROR)


}