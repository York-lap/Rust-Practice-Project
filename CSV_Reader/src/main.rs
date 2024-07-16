/*
 * @Author: yk
 * @Date: 2024-07-16 14:29:38
 * @Description: Just a simple case to use csv crate for reading
 */
extern crate csv;

use std::error::Error;


fn read_from_csv(path: &str) -> Result<(),Box<dyn Error>>{
    
    let mut reader = csv::Reader::from_path(path)?;

    for line in reader.records(){

        let record = line?;

        print!("{:?}",record);
    }

    Ok(())
}

fn main(){
    if let Err(e) = read_from_csv("./job_status.csv"){
        eprintln!("{:#?}",e);
    }
}


