use std::fs::File;
use std::io::{BufReader, BufRead};

struct Line {
    id: i32,
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32, 
    m: f64
}


pub fn main(file_name: String) -> std::io::Result<()> {
    // Read the lines 
    let lines = read_lines(&file_name)?;

    // Calc the number of overlapping points 
    let mut overlapping = 0;


    // Finished 
    println!("{}", overlapping);

    Ok(())
}


fn read_lines(file_name: &String) -> std::io::Result<Vec<Line>> {
    // Read file 
    let file = File::open(file_name)?;
    let br = BufReader::new(file);

    let mut id = 0;
    let mut lines: Vec<Line> = Vec::new();

    // Parse Lines 
    for line in br.lines().skip(1) {
        // Get the two points 
        let points = line.unwrap();
        let points: Vec<&str> = points.split(" -> ").collect();
        let p1 = parse_point(&points[0]);
        let p2 = parse_point(&points[1]);

        // Calculate the gradient 
        let deltax = p1[0] - p1[1];
        let deltay = p2[0] - p2[1];

        let m = match deltax {
            0 => 90 as f64,
            _ => ((deltay / deltax) as f64).atan()
        };

        // Check if hor or vertical 
        if m == 0 as f64 || m == 90 as f64 {
            //println!("{},{} -> {},{} : {}", p1[0], p1[1], p2[0], p2[1], m);
            lines.push(Line {
                id: id,
                x1: p1[0],
                x2: p2[0],
                y1: p1[1],
                y2: p2[1], 
                m: m
            });

            id += 1;
        }
    }


    Ok(lines)
}


fn parse_point(p: &str) -> [i32; 2] {
    // Get two values of point 
    let values: Vec<&str> = p.split(',').collect();
    let x: i32 = values[0].parse().unwrap();
    let y: i32 = values[1].parse().unwrap();

    return [x, y];
}