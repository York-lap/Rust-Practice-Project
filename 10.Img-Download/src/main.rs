/*
 * @Author: yk
 * @Date: 2024-07-18 08:21:54
 * @Description: This is a simple case to use Async+Await download img to tempfile
 */

use error_chain::error_chain;
use std::io::copy;
use std::fs::File;
use tempfile::Builder;

error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    // 1.Create the new tempfolder

    let temp_dir = Builder::new().prefix("example").tempdir()?;
    
    //2.Set the download URL
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";

    //3. Send the `GET` Request and Get the response
    let response = reqwest::get(target).await?;


    //4.Create the file ---- (to save img)
    let mut dest = {

        // fname : dest_path
        let fname = response 
            .url()
            .path_segments()
            .and_then(|segments| segments.last()) // Get the last seg(filename)
            .and_then(|name| if name.is_empty(){None}else{Some(name)})
            .unwrap_or("tmp.bin");

        println!("File to download: `{}`",fname);
        let fname = temp_dir.path().join(fname);
        println!("will be loacated unser: `{:?}`",fname);
        File::create(fname)?

    };

    let content = response.text().await?;
    copy(&mut content.as_bytes(),&mut dest)?;
    Ok(())


}