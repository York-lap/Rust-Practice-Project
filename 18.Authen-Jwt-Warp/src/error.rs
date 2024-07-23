/*
 * @Author: yk
 * @Date: 2024-07-23 10:48:59
 * @Description: 
 */
use thiserror::Error;

 // 2.4 Set Error mole
 #[derive(Error, Debug)]
pub enum Error{
    #[error("jwt token create error")]
    JWTCREATEERROR,
    #[error("wrong credentials")]
    WrongCredentialsError,

}

impl warp::reject::Reject for Error{}