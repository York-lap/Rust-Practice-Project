/*
 * @Author: yk
 * @Date: 2024-07-18 09:46:50
 * @Description: This is a simple case to use `select` to extract the links in website
 */
use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain!{
    foreign_links{
        IoError(std::io::Error);
        ReqError(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()>{

    // 1.Get the WebSite`s html text

    let res = reqwest::get("https://www.rust-lang.org/zh-CN/")
        .await?
        .text()
        .await?;

    // 2.Parse the text <a href = links>

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}",x));

    Ok(())
}