/*
 * @Author: yk
 * @Date: 2024-07-20 14:11:36
 * @Description: 
 * 
 */
mod db;
use db::*;


use std::io;
// Func:  CLearn the screen
fn clr() {
    println!("{}[2J",27 as char);
}



fn main()
{
    clr();
    let conn = init_database().expect("Failed to initialize the dataase");
    let ascii = r#"
    
            ##   ##                     ##                 ###                   
            ##  ##                     ##                  ##                     
            ####      ####    ## ###   ##  ##             ##       #####    ####  
            ##     ##   ##  ####     ## ##               ##     ##   ##   ##  ## 
            ##    ##    ##  ##       ####               ##     ##    ##  ##   ## 
            ##     ##   ##  ##       ##  ##            ##     ##  ###   ##  ##  
            ##      ####    ##       ##   ##          ####     ### ##  #####    
                                                                      ##                                                                                                                                                                          
           Made By————————💗💗York-lap💗💗   Take care of your passwords!🙋  
           👏👏https://github.com/York-lap/Rust-Practice-Project                                                                    
    "#;
    
    println!("{}",ascii);

    loop {
        
        println!("====== Password Ault Menu=====");
        println!("1.Add Entry");
        println!("2.List Entries");
        println!("3.Search");
        println!("4.Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim(){
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    promt("Server: "),
                    promt("User: "),
                    promt("Password: "),
                );

                write_pwd_to_db(&conn, &entry.server, &entry.user, &entry.password)
                    .expect("Failed to write into database");

                println!("Successfully to write into database!");


            }
            "2" => {
                clr();
                let services = read_pwd_from_db(&conn).unwrap_or_else(|err|{
                    eprintln!("Error reading passwords: {}",err);
                    Vec::new()
                });

                for item in &services {
                    println!("Server: {}\nUser: {}\nPassword: {}\n",item.server,item.user,item.password);
                }
            }
            "3" => {
                clr();
                let search = promt("Search by server name: ");
                match search_pwd_from_db(&conn, &search) {
                    Ok(Some(entry)) => {
                        println!("Server: {}\nUser: {}\nPassword: {}\n",entry.server,entry.user,entry.password);
                    }
                    Ok(None) => {
                        println!("Server not found!");
                    }
                    Err(e) => {
                        eprintln!("Error searching for server: {}",e);
                    }
                }
            }
            "4" => {
                clr();
                println!("GoodBye!");
                break;
            }
            _ => println!("Invalid Choice"),
        }


    }
}