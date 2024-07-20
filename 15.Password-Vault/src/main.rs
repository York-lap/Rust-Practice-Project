/*
 * @Author: yk
 * @Date: 2024-07-19 15:13:52
 * @Description: 
 */
mod pentry;

use crate::pentry::ServiceInfo;
use crate::pentry::{read_pwd_frm_file,prompt};

// Func:clear the window
fn clr(){
    print!("{}[2J",27 as char);

}

fn main() {
    clr();
    
    let ascii = r#"
  ##  ##                      ##                ###                     
  ##  ##                     ##                  ##                     
  ####      ####    ## ###   ##  ##             ##       #####    ####  
   ##     ##   ##  ####     ## ##               ##     ##   ##   ##  ## 
   ##    ##    ##  ##       ####               ##     ##    ##  ##   ## 
  ##     ##   ##  ##       ##  ##              ##     ##  ###   ##  ##  
  ##      ####    ##       ##   ##            ####     ### ##  #####    
                                                               ##                                                                                                                                                                          
    "#;
    
    println!("{}",ascii);

    loop {
        println!("======Password Manager Menu======");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Serach");
        println!("4. Quit");

        let mut choice =  String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim(){
            "1" => {    // 1.Add Entry
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service："),
                    prompt("user："),
                    prompt("password："),
                );

                println!("Entry added successfullt");
                entry.write_to_file();

            }
            "2" => {    // 2.List Entries
                clr();
                let services  = read_pwd_frm_file().unwrap_or_else(|err|{
                    eprintln!("Error reading pwd: {}",err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "\n- Service: {} - \n- User: {} -\n- Passwprd {}  -",
                    item.service,item.user,item.password
                    );
                }
            }
            "3" => {    // 3.Search
                clr();
                let services = read_pwd_frm_file().unwrap_or_else(|err|{
                    eprintln!("Error reading passwords: {}",err);
                    Vec::new()
                });

                let search = prompt("search: ");
                for item in &services{
                    if item.service.as_str() == search.as_str() {
                        println!(
                            "\n- Service: {} - \n- User: {} -\n- Passwprd {}  -",
                        item.service,item.user,item.password
                        );
                    }
                }


            }
            "4" => {
                clr();
                println!("Good Bye");
                break;
            }
            _ => println!("Invalid choice"),
        }

        print!("\n\n");
    }
}