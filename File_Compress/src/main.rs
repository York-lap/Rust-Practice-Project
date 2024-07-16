/*
 * @Author: yk
 * @Date: 2024-07-16 11:28:16
 * @Description: File Compress[DgzEncoder]
 */
extern crate flate2;

use std::io::{BufReader,copy};
use std::fs::File;
use std::time::Instant;
use flate2::Compression;
use flate2::write::GzEncoder; 
use std::env::args;


fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `Source` `Target`");
        return    
    }

    // 1.Turn the Source into BufReader

    let mut input = BufReader::new(
        File::open(
            args().nth(1).unwrap()
        ).unwrap()
    );

    // 2.Create the output(File) for the `Target`

    let output = File::create(
        args().nth(2).unwrap()
    ).unwrap();

    // 3.Set the Encoder
    let mut encoder = GzEncoder::new(
        output,  // You want to write (File) 
        Compression::default(), // Your Encoder's Level
    );

    // 4.Copy the input to encoder(1) & Accept the Compress's File (2)

    let start = Instant::now(); //Record the time
    copy(&mut input, &mut encoder).unwrap(); // (1)  
    let output = encoder.finish().unwrap(); // (2)

    // 5.Out put Running Info

    println!("Sourse Len: {:?}",input.get_ref().metadata().unwrap().len());
    println!("Target Len: {:?}",output.metadata().unwrap().len());
    println!("Elapsed: {:?}",start.elapsed());

}

// You can use z-shell Create a test.txt
//touch test.txt
// for ((i=1;i<=10000;i++)); do echo 'a' >> test.txt; done