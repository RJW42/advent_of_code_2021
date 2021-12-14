use std::fs::File;
use std::io::{BufReader, BufRead};


struct Heightmap {
    width: u32,
    height: u32,
    store: Vec<i32>,
}


pub fn main(file_name: String) -> std::io::Result<()> {
    // Read file 
    let heightmap = read_heightmap(file_name)?;
    

    // Finished 
    Ok(())
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
        let line = line?;

        width = line.len();
        height += 1;

        for c in line.chars() {
            match c.to_digit(10) {
                Some(a) => store.push(a as i32),
                None => ()
            }
        }
    }

    // Finished 
    Ok(Heightmap {
        width: width as u32,
        height: height as u32,
        store: store
    })
}