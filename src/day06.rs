use std::fs::File;
use std::io::{BufReader, BufRead};


pub fn main(file_name: String) -> std::io::Result<()> {
    // Get Fish 
    let mut fish = get_inital_fish(&file_name)?;
    let num_days = 256;

    // Calc the number of fish after num_days 
    for _i in 0..num_days {
        let tmp = fish[0];

        // Decrease all fish by 1 
        for i in 1..9 {
            fish[i - 1] = fish[i];
        }

        // Deal with fish which are having kids 
        fish[6] += tmp;
        fish[8] = tmp;

        /*
        for i in 0..9 {
            for j in 0..fish[i] {
                print!("{},", i);
            }
        }
        println!(); */
    }

    // Calc number of fish
    let num_fish: u64 = fish.iter().sum();

    println!("{}", num_fish);


    // Finished 
    Ok(())
}


fn get_inital_fish(file_name: &String) -> std::io::Result<[u64; 9]> {
    let file = File::open(file_name)?;
    let mut br = BufReader::new(file);

    // Parse file to get number of each fish  
    let mut line = String::new();
    let mut fish: [u64; 9] = [0; 9];
    
    br.read_line(&mut line)?;

    for element in line.split(',') {
        let element: usize = element.parse().unwrap();

        fish[element] += 1;
    }


    // Finished Parsing
    Ok(fish)
}   