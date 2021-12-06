use std::fs::File;
use std::io::{BufReader, BufRead};


pub fn main(file_name: String) -> std::io::Result<()> {
    // Read file 
    let file = File::open(&file_name)?;
    let br = BufReader::new(file);
    let no_bits: u32 = 12;

    // Parse file and calculate results 
    let mut lines: Vec<String> = Vec::new();


    for line in br.lines(){
        let line = line.unwrap();

        lines.push(line);
    }


    let oxygen_result = get_result(&lines, no_bits, true);
    let c02_result = get_result(&lines, no_bits, false);

    println!("{} {}", oxygen_result, c02_result);
    println!("{}", oxygen_result * c02_result);
    

    /* PART 1 
    // Init bit store 
    let mut bits: [i32; 5] = [0; 5];
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

    } */

    // Finished 
    Ok(())
}


fn get_result(lines_in: &Vec<String>, no_bits: u32, is_oxygen: bool) -> u32{
    let mut result: u32 = 0;
    let mut lines: Vec<String> = Vec::new();
    let base: u32 = 2;


    for line in lines_in{
        lines.push(line.to_string());
    }


    for i in 0..no_bits {
        // Init vars 
        let mut new_lines: Vec<String> = Vec::new();
        let mut no_sig_bits = 0;

        // First find most common bit 
        for line in &lines {
            if line.chars().nth(i as usize).unwrap() == '1' {
                no_sig_bits += 1;
            }
        }

        // Keep all lines which match the critiera 
        for line in &lines {
            // Check char 
            let c = line.chars().nth(i as usize).unwrap();

            // Check if matches 
            if (no_sig_bits as f32) >= ((lines.len() as f32) / 2.0) {
                // 1 Most common 
                if c == '1' && is_oxygen {
                    new_lines.push(line.to_string());
                } else if c == '0' && !is_oxygen {
                    new_lines.push(line.to_string());
                }
            } else {
                // 0 Most common 
                if c == '0' && is_oxygen {
                    new_lines.push(line.to_string());
                } else if c == '1' && !is_oxygen {
                    new_lines.push(line.to_string());
                }
            }
        }

        // Set new lines as new lines
        lines = new_lines;

        // Check for solution
        if lines.len() == 1 {
            for (j, c) in lines[0].chars().enumerate() {
                if c == '1' {
                    result += base.pow((no_bits - 1) - (j as u32));
                }
            }

            break;
        }
    }

    result
}