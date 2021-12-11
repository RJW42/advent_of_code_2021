use std::fs::File;
use std::io::{BufReader, BufRead};


struct SignalEntry {
    signal_code: Vec<[bool; 7]>, // 10 
    signal_value: Vec<[bool; 7]>, // 4
}


pub fn main(file_name: String) -> std::io::Result<()> {
    // Get segment codes 
    let signals = get_codes(&file_name)?;
    let mut result = 0;

    // Calc result 
    for signal in signals {
        for value in signal.signal_value {
            result += match value.iter().fold(0, |sum, val| sum + (*val as u32)) {
                7 => 1, // 8
                3 => 1, // 7
                4 => 1, // 4
                2 => 1, // 1
                _ => 0
            };
        }
    }

    println!("{}", result);

    // Finished 
    Ok(())
}


fn get_codes(file_name: &String) -> std::io::Result<Vec<SignalEntry>> {
    let file = File::open(file_name)?;
    let br = BufReader::new(file);
    let mut signals: Vec<SignalEntry> = Vec::new();

    // Parse Codes 
    for line in br.lines() {
        // Parse Line into code and value 
        let line = line?;
        let components = line.split(" | ").collect::<Vec<&str>>();
        let code = components[0];
        let value = components[1];

        // Parse code and value 
        let codes = get_code(code);
        let values = get_code(value);

        // Add mew signal entry to output 
        signals.push(SignalEntry {
            signal_code: codes,
            signal_value: values
        });
    }

    // Finished Parsing
    Ok(signals)
}


fn get_code(code_str: &str) -> Vec<[bool; 7]>{
    let codes = code_str.split(" ").collect::<Vec<&str>>();
    let mut output: Vec<[bool; 7]> = Vec::new();
    
    for code in codes {
        let mut code_arr = [false; 7];

        for c in code.chars() {
            let i = (c as u32) - 97;

            code_arr[i as usize] = true;
        }

        output.push(code_arr);
    }

    output
}