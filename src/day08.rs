use std::fs::File;
use std::io::{BufReader, BufRead};


struct SignalEntry {
    signal_code: Vec<[bool; 7]>, // 10 
    signal_value: Vec<[bool; 7]>, // 4
    combined: Vec<[bool; 7]>, // 14
}


pub fn main(file_name: String) -> std::io::Result<()> {
    // Get segment codes 
    let signals = get_codes(&file_name)?;

    // Calc result 
    for signal in signals {
        // Figure out how to display each number 
        let mut decoder = [[false; 7]; 10];
        let mut remaning = Vec::<[bool; 7]>::new();

        let mut display = [0; 10];
        /* 0 -> Top
           1 -> Top Left
           2 -> Top Right
           3 -> Middle
           4 -> Bottom Left
           5 -> Bottom Right
           6 -> Bottom 
        */

        for value in signal.combined {
            match (&value).iter().fold(0, |sum, val| sum + (*val as u32)) {
                7 => decoder[8] = value, // 8
                3 => decoder[7] = value, // 7
                4 => decoder[4] = value, // 4
                2 => decoder[1] = value, // 1
                _ => remaning.push(value)
            };
        }

        // Get Top Value from diference between 7,1
        for i in 0..7 {
            if (decoder[7][i] == true) && (decoder[1][i] == false) {
                display[0] = i;
                break;
            }
        }

        // Get Bottom value from value of size 5 with diff 1 on set of elements in (4 + Top).
        // Also Gets Values 2, 9 
        let mut set_4_top: [bool; 7] = decoder[4].clone();
                set_4_top[display[0]] = true;
        let mut new_remaning = Vec::<[bool; 7]>::new();


        for val in remaning {
            let size: u32 = (&val).iter().fold(0, |sum, v| sum + (*v as u32));
            let diff: u32 = (&val).iter().zip((&set_4_top).iter()).fold(0, |sum, (a, b)| sum + (*a == !*b) as u32);
            print!("{}:{}, ", size, diff);

            if size == 6 && diff == 3 {
                // Got 9. Extract Bottom Value 
                decoder[9] = val;
                continue;
            }

            if size == 5 && diff == 4 {
                // Got 2
                decoder[2] = val;

                continue;
            }

            new_remaning.push(val);
        }
        println!();

        remaning = new_remaning;


        // Get 0, 3, 5, 6 Using the size of the set and its difference with 1 
        for val in remaning {
            let size: u32 = (&val).iter().fold(0, |sum, val| sum + (*val as u32));
            let diff: u32 = (&val).iter().zip((&decoder[1]).iter()).fold(0, |sum, (a, b)| sum + (*b && !*a) as u32);

            match (size, diff) {
                (5, 0) => decoder[3] = val, // 3
                (5, 1) => decoder[5] = val, // 5
                (6, 0) => decoder[0] = val, // 0
                (6, 1) => decoder[6] = val, // 6
                _ => (),
            };
        }

        // Get The Value of the output 
        for digit in signal.signal_value {
            for (i, decode) in (&decoder).iter().enumerate() {
                if &digit[..] == &decode[..] {
                    print!("{}", i);
                }
            }
            print!(" ");
        }
        println!();
    }

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
        let mut combined = values.to_vec();
        combined.extend(codes.to_vec());

        // Add mew signal entry to output 
        signals.push(SignalEntry {
            signal_code: codes,
            signal_value: values,
            combined: combined
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