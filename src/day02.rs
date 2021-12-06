use std::fs::File;
use std::io::{BufReader, BufRead};


pub fn main(file_name: String) -> std::io::Result<()> {
    // Read file 
    let file = File::open(file_name)?;
    let br = BufReader::new(file);

    // Parse file and calculate results 
    let mut horizontal = 0;
    let mut depth = 0; 
    let mut aim = 0;

    for line in br.lines(){
        // Ignore errors
        let mut line = line.unwrap();

        // Check what type this line is 
        if line.starts_with("up") {
            aim -= get_int(line.split_off(3));
        } else if line.starts_with("down") {
            aim += get_int(line.split_off(5));
        } else if line.starts_with("forward") {
            let x = get_int(line.split_off(8));
            horizontal += x;
            depth += aim * x;
        }
    }


    let result = depth * horizontal;

    println!("Result: {}", result);

    // Finished 
    Ok(())
}


fn get_int(line: String) -> u32 {
    let val = match line.parse::<u32>() {
        Ok(n) => n,
        _ => panic!("Failed to parse int in file"),
    };

    val
}