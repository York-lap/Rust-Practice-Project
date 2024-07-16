/*
 * @Author: yk
 * @Date: 2024-07-16 13:40:57
 * @Description: File_Decompress [GzDecoder]
 */
extern crate flate2;

use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{copy,BufWriter,BufReader};
use std::env::args;
use std::time::Instant;

fn main() {
    
    if args().len() != 3{
        eprintln!("Usage: `Source` `Target`");
        return 
    }

    // 1.Turn the CompressFile into BufWriter

    let input = BufReader::new(
        File::open(
            args().nth(1).unwrap()
        ).unwrap()
    );

    // 2.Create a file for the Decompress

    let mut output = BufWriter::new(
        File::create(
            args().nth(2).unwrap()
        ).unwrap()
    );

    // 3.Set the decoder

    let mut decoder = GzDecoder::new(input);

    // 4.Decoder
    let start = Instant::now();
    copy(&mut decoder,&mut output).unwrap();

    // 5.Show Info
    println!("Target Len: {:?}",output.get_ref().metadata().unwrap().len());
    println!("Elapsed: {:?}",start.elapsed());

}

// You can run:  cargo run CompressTest DecompressTest.txt
