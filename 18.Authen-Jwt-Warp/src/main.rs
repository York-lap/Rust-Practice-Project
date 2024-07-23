/*
 * @Author: yk
 * @Date: 2024-07-23 10:31:06
 * @Description: 
 */
use std::{collections::HashMap, sync::Arc};
use serde::{Deserialize,Serialize};
mod error;
use error::Error;
mod auth;
use auth::{Role,create_jwt};
use warp::{reject::{self, Rejection}, reply::{self, Reply}, Filter};
 ///    1. Set the dependencies
 ///    2. Build the Data modle ---- User, Users, LoginRequest, LoginResponse, Error
 ///    3. Init the data: users
 ///    4. Set the routes
 ///    5. Run


 // 2.1 User model & Users
 #[derive(Clone, Serialize, Deserialize)]
 pub struct User{
    uid: String,
    email: String,
    pwd: String,
    role: String
 }
 // impl `new `
 impl User {
    fn new(uid:String,email: String,pwd: String,role: String)->Self{
        User { uid: (uid), email: (email), pwd: (pwd), role: (role) }
    }
 }
type Users = Arc<HashMap<String,User>>;

// 2.2 Login Request (by email, pwd) 
#[derive(Deserialize)]
 pub struct LoginRequest{
    email: String,
    pwd: String,
 }

 // 2.3 Login Response
 #[derive(Serialize)]
 pub struct LoginResponse{
    token: String,
 }


 // 3.1 Init data model: users
 fn init_users() -> HashMap<String,User>{
    let mut map = HashMap::new();

    map.insert(
        String::from("1"),
        User::new
        (
    String::from("1"),
  String::from("123@qq.com"),
    String::from("1234"),
   String::from("User")
        ),
    );
    map.insert(
        String::from("2"),
        User::new
        (
    String::from("2"),
  String::from("123@163.com"),
    String::from("4321"),
   String::from("Admin")
        ),
    );
    map
 }

 #[tokio::main]
 async fn main(){

    let users: Users = Arc::new(init_users());

    // 4.1 Set the route: `login_route`
    let login_route = warp::path("login")
        .and(warp::post())
        .and(warp::any().map(move || users.clone()))
        .and(warp::body::json())    // json -> rust
        .and_then(login_handle);    // func   

    warp::serve(login_route).run(([127,0,0,1],8000)).await;
 }


// 4.2 Func: login_handle
async fn login_handle(users: Users, body: LoginRequest) -> Result<impl Reply,Rejection>{
    
    match users
        .iter()
        .find(|(_uid,user)| user.email == body.email && user.pwd == body.pwd)
        {
            Some((uid,user)) => {
                let token = create_jwt(&uid, &Role::from_str(&user.role))
                   .map_err(|e| reject::custom(e))?;
                Ok(reply::json(&LoginResponse {token}))
                

            }
            None => Err(reject::custom(Error::WrongCredentialsError))
        }
    
}