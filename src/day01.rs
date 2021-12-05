use std::fs::File;
use std::io::{BufReader, BufRead};


pub fn main(file_name: String) -> std::io::Result<()> {
    // Read file 
    let file = File::open(file_name)?;
    let mut br = BufReader::new(file);

    // Parse file and calculate results 
    let mut line = String::new();
    let mut result = 0;
    
    br.read_line(&mut line)?;

    let mut previous_line = match line.trim().parse::<u32>() {
        Ok(n) => n,
        _ => panic!("Failed to parse int in file"),
    };

    for line in br.lines(){
        // Get the current line 
        let current_line = match line?.parse::<u32>() {
            Ok(n) => n,
            _ => panic!("Failed to parse int in file"),
        };

        // Check if it is increasing 
        if current_line > previous_line {
            result += 1;
        }

        previous_line = current_line;
    }

    println!("Result: {}", result);

    // Finished 
    Ok(())
}