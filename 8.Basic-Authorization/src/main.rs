/*
 * @Author: yk
 * @Date: 2024-07-17 15:10:13
 * @Description: This is a simple case to use basic_auth
 */

 use reqwest::Error;
 use reqwest::blocking::Client;

 fn main() -> Result<(),Error> {
    
    let user = "test_user".to_string();
    let passwd: Option<String> = None;

    let client = Client::new();

    let response = client
        .get("http://httpbin.org/")
        .basic_auth(user, passwd)
        .send();

    print!("{:#?}",response);

    Ok(())
 }
