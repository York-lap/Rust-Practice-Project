use std::{fs::OpenOptions, io::BufRead};
use std::io;
use std::io::Write;
use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct ServiceInfo{
    pub service: String,
    pub user: String,
    pub password: String
}

impl ServiceInfo {
    pub fn new(service: String, user: String, password: String )->Self{
        ServiceInfo{
            service,
            user,
            password
        }
    }
    fn to_json(&self)-> String{
        serde_json::to_string(&self).expect("Failed to serialize to Json")
    }

    pub fn write_to_file(&self){
        let json_output = format!("{}\n",self.to_json());

        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("passwords.json")
        {
            Ok(mut file) => {
                if let Err(e) =  file.write_all(json_output.as_bytes()) {
                    eprintln!("Failed to write in passwords.json: {}",e);
                }else{
                    println!("Successfully write into password.josn");
                }

            }
            Err(e) => eprintln!("Error: {}",e),
        }


    }
    pub fn from_json(json_string: &str) -> Result<Self,serde_json::Error>{
        serde_json::from_str(json_string)
    }  
}

pub fn read_pwd_frm_file()->Result<Vec<ServiceInfo>,std::io::Error>{
        
    let file = File::open("passwords.json").unwrap();
    let reader = io::BufReader::new(file);
    
    let mut services = Vec::new();

    for line in reader.lines(){
        if let Ok(json_string) =  line{
            if let Ok(service_info) = ServiceInfo::from_json(&json_string){
                services.push(service_info);
            }            
        }

    }

    Ok(services)
    
}

pub fn prompt(prompt: &str) -> String{
    print!("{}",prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()

}