use std::fs::File;
use std::io::{BufReader, BufRead};


pub fn main(file_name: String) -> std::io::Result<()> {
    // Read file 
    let file = File::open(file_name)?;
    let br = BufReader::new(file);
    let no_bits: u32 = 12;

    // Parse file and calculate results 

    // Init bit store 
    let mut bits: [i32; 12] = [0; 12];
    let mut no_lines = 0;

    // Populate bit store 
    for line in br.lines() {
        let line = line.unwrap();

        for (i, c) in line.chars().enumerate(){
            if c == '1' {
                bits[i] += 1;
            }
        }

        
        no_lines += 1;
    }

    // Calculate epsilon and gama rate 
    let mut epsilon = 0;
    let mut gamma = 0;
    let base: i32 = 2;

    for i in 0..no_bits {
        if bits[i as usize] > (no_lines / 2) {
            gamma += base.pow((no_bits - 1) - (i as u32));
        } else { 
            epsilon += base.pow((no_bits - 1) - (i as u32));
        }

    }

    println!("Result: {} ", gamma * epsilon);

    // Finished 
    Ok(())
}
