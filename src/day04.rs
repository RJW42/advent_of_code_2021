use std::fs::File;
use std::io::{BufReader, BufRead};


struct Board {
    val_store: [u32; 25],
    set_store: [bool; 25],
}


pub fn main(file_name: String) -> std::io::Result<()> {
    // 

    // Finished 
    Ok(())
}


pub fn read_boards(file_name: String) -> std::io::Result<()> {
    // Read file 
    let file = File::open(file_name)?;
    let br = BufReader::new(file);
    let mut line_count = 0;
    let mut curr_board: [String; 5] = // TODO FIniish. Convert this to a vector. 


    // Parse Boards 
    for line in br.lines().skip(1) {
        // First Line Should be White space 
        if line_count == 0 {
            continue;
        }

        // Build board 


        // Increment Line count 
        line_count++;
    }

    Ok(())
}