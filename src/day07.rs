use std::fs::File;
use std::io::{BufReader, BufRead};


pub fn main(file_name: String) -> std::io::Result<()> {
    // Get Crabs 
    let mut crabs = get_crab_pos(&file_name)?;
    let median = median(&mut crabs);
    let mean = mean(&crabs);
    let var: i32 = mean as i32;

    println!("Median: {} Mean:{} Len:{}", median, mean, crabs.len());

    // Calc result 
    let mut result = 0;
    let mut result_val = 0;

    for i in (var - 5)..(var + 5) {
        if i < 0 || i > crabs.len() as i32 {
            continue;
        }

        let mut tmp = 0;

        for element in &crabs {
            let moves = (i - *element as i32).abs() as u32;

            tmp += (moves * (moves + 1)) / 2;
        }

        println!("{}, {}", i , tmp);

        if tmp < result || result == 0 {
            result = tmp;
            result_val = i;
        }
    }

    println!("Best Fuel: {} Best Pos: {}", result, result_val);


    // Finished 
    Ok(())
}


fn median(crabs: &mut Vec<u32>) -> u32{
    crabs.sort();

    let mid = crabs.len() / 2;

    if crabs.len() % 2 == 0{
        (crabs[mid - 1] + crabs[mid]) / 2
    } else {
        crabs[mid]
    }
}


fn mean(crabs: &Vec<u32>) -> u32 {
    let total: u32 = crabs.iter().sum();

    (total as f32 / (crabs.len() as f32)).round() as u32    
}


fn get_crab_pos(file_name: &String) -> std::io::Result<Vec<u32>> {
    let file = File::open(file_name)?;
    let mut br = BufReader::new(file);

    // Parse file to get pos of each crab 
    let mut line = String::new();
    let mut crabs: Vec<u32> = Vec::new();
    
    br.read_line(&mut line)?;

    for element in line.split(',') {
        let element: u32 = element.parse().unwrap();

        crabs.push(element);
    }


    // Finished Parsing
    Ok(crabs)
}   