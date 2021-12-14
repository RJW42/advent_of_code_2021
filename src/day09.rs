use std::fs::File;
use std::io::{BufReader, BufRead};


struct Heightmap {
    width: u32,
    height: u32,
    store: Vec<Vec<i32>>,
}

const ADJ: [[i32; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];


pub fn main(file_name: String) -> std::io::Result<()> {
    // Read file 
    let hm = read_heightmap(file_name)?;

    // Find low points 
    let mut result = 0;

    for x in 0..hm.width {
        'outer: for y  in 0..hm.height {
            for offset in ADJ {
                let off_x:i32 = (x as i32) + offset[0];
                let off_y:i32 = (y as i32) + offset[1];

                if off_x < 0 || off_x >= hm.width as i32 || off_y < 0 ||  off_y >= hm.height as i32 {
                    continue;
                }

                if hm.store[y as usize][x as usize] >= hm.store[off_y as usize][off_x as usize] {
                    continue 'outer;
                }
            }

            // Found minimum
            //result += hm.store[y as usize][x as usize] + 1;

            // Find the size of this basin 
            let basin_size = get_basin_size(&hm, x as u32, y as u32);

            println!("{} : {}", hm.store[y as usize][x as usize], basin_size);

            result = result * basin_size;
        }
    }

    println!("{}", result);
    

    // Finished 
    Ok(())
}


fn get_basin_size(hm: &Heightmap, x: u32, y: u32) -> u32 {
    0
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