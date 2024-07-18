/*
 * @Author: yk
 * @Date: 2024-07-17 11:21:46
 * @Description: This is a simple based on `Gest_Requst` but use the async+await
 *          you need to read official documents like async and so on.
 */
use error_chain::error_chain;

error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main()-> Result<()> {
    let res = reqwest::get("https://reqres.in/api/users/2").await?;
    println!("Status: {}",res.status());
    println!("Headers:\n{:#?}",res.headers());

    let body = res.text().await?;
    println!("Body:\n {}",body);
    Ok(())
}
