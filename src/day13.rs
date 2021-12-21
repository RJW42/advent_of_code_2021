use std::fs::File;
use std::io::{BufReader, BufRead};


struct Point {
    x: u32,
    y: u32
}


struct Contents {
    points: Vec<Point>,
    x_folds: Vec<u32>,
    y_folds: Vec<u32>
}


pub fn main(file_name: String) -> std::io::Result<()> {
    // Read file contents 
    let mut content = get_contents(file_name)?;



    // Finished 
    Ok(())
}


fn get_contents(file_name: String) -> std::io::Result<Contents> {
    // Read file
    let file = File::open(file_name)?;
    let br = BufReader::new(file);

    // Init vars 
    let mut points = Vec::new();
    let mut x_folds = Vec::new();
    let mut y_folds = Vec::new();
    let mut in_points = true;

    // Parse file 
    for line in br.lines(){
        // Check for errors 
        let line = line?;

        if in_points {
            // Check if file is blank 
            if line.len() == 0 {
                in_points = false;
                continue;
            }

            parse_point(&mut points, line);
        } else {
            // Parse fold 
            parse_fold(&mut x_folds, &mut y_folds, line);
        }

    }


    // Finished 
    Ok(Contents{
        points: points,
        x_folds: x_folds,
        y_folds: y_folds 
    })
}


fn parse_point(points: &mut Vec<Point>, line: String){
    let mut line = line.split(",").collect::<Vec<&str>>();

    points.push(Point {
        x: line[0].parse().expect("Error not num"),
        y: line[1].parse().expect("Error not num")
    })
}


fn parse_fold(x_folds: &mut Vec<u32>, y_folds: &mut Vec<u32>, line: String) {
    let line = line.replace("fold along ", "");
    let mut line = line.split("=").collect::<Vec<&str>>();


    match line[0].chars().nth(0).unwrap() {
        'y' => y_folds.push(line[1].parse().expect("Error not num")),
        'x' => x_folds.push(line[1].parse().expect("Error not num")),
        _ => ()
    }
}