use std::fs::File;
use std::io::{BufReader, BufRead};


pub fn main(file_name: String) -> std::io::Result<()> {
    // Get Crabs 
    let mut crabs = get_crab_pos(&file_name)?;
    let median = median(&mut crabs);

    println!("{} {}", median, crabs.len());

    // Calc result 
    let mut result = 0;

    for element in crabs {
        result += (median as i32 - element as i32).abs() as u32;
    }

    println!("{}", result);


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