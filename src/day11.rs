use std::fs::File;
use std::io::{BufReader, BufRead};


struct Heightmap {
    width: u32,
    height: u32,
    store: Vec<Vec<i32>>,
}


const ADJ: [[i32; 2]; 8] = [[0, 1], [0, -1], [1, 0], [-1, 0], [1, 1], [-1, -1], [1, -1], [-1, 1]];
const STEPS: u32 = 1000;


pub fn main(file_name: String) -> std::io::Result<()> {
    // Read file 
    let mut hm = read_heightmap(file_name)?;
    let mut result = 0;

    // Calc result 
    for i in 0..STEPS {
        let r = perform_step(&mut hm);

        if r == (hm.width * hm.height) {
            result = i + 1;
            break;
        }
    }

    println!("Result: {}", result);

    // Finished 
    Ok(())
}


fn perform_step(hm: &mut Heightmap) -> u32 {
    // Count Number of Flashes 
    let mut flashes = 0;
    let mut can_flash = false;

    // Increase all octupuses by 1 
    for x in 0..hm.width {
        for y in 0..hm.height {
            hm.store[y as usize][x as usize] += 1;

            if hm.store[y as usize][x as usize] > 9 {
                can_flash = true;
            }
        }
    }

    //print_hm(hm);
    //println!();


    // While octupuses can flash make them flash 
    while can_flash {
        can_flash = false; // Reset 

        // Create a copy of the store to contain updates 
        let mut new_store: Vec<Vec<i32>> = hm.store.to_vec();

        for x in 0..hm.width {
            for y in 0..hm.height {
                if hm.store[y as usize][x as usize] <= 9 {
                    // Can't Flash 
                    continue;
                }

                // Flash this octupus 
                new_store[y as usize][x as usize] = 0;

                flashes += 1;

                // Update all adjacent octopi 
                for offset in ADJ {
                    let new_x = x as i32 + offset[0];
                    let new_y = y as i32 + offset[1];

                    if new_x < 0 || new_x >= hm.width as i32 || new_y < 0 || new_y >= hm.height as i32 {
                        continue;
                    }

                    // Increase this octupus score by 1 
                    let v = new_store[new_y as usize][new_x as usize];

                    new_store[new_y as usize][new_x as usize] = 
                        (v > 0) as i32 * (v + 1); // + (v == 0) as i32 * (-1) + (v < 0) as i32 * (v - 1);

                    if new_store[new_y as usize][new_x as usize] > 9 {
                        can_flash = true;
                    }
                }   
            }
        }

        // Update store 
        hm.store = new_store;
    }

    // Print Squid after flashes 
    print_hm(hm);
    println!();


    flashes 
}



fn print_hm(hm: &Heightmap){
    for x in 0..hm.width {
        for y in 0..hm.height {
            print!("{} ", hm.store[y as usize][x as usize]);
        }
        println!();
    }
}



fn read_heightmap(file_name: String) -> std::io::Result<Heightmap>{
    // Init store 
    let mut store = Vec::new();
    let mut width = 0;
    let mut height = 0;

    // Read in each line of the store 
    let file = File::open(file_name)?;
    let br = BufReader::new(file);

    for line in br.lines(){
        let mut line_store = Vec::new();
        let line = line?;

        width = line.len();
        height += 1;

        for c in line.chars() {
            match c.to_digit(10) {
                Some(a) => line_store.push(a as i32),
                None => ()
            }
        }

        store.push(line_store);
    }

    // Finished 
    Ok(Heightmap {
        width: width as u32,
        height: height as u32,
        store: store
    })
}