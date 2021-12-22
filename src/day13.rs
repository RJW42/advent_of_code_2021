use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;


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

    // Perform y_folds 
    for y_fold in &content.y_folds {
        for i in 0..content.points.len() {
            if content.points[i].y > *y_fold {
                content.points[i].y = *y_fold - (content.points[i].y - *y_fold); 
            }
        }
    }


    // Perform x_folds 
    
    for x_fold in &content.x_folds {
        for i in 0..content.points.len() {
            if content.points[i].x > *x_fold {
                content.points[i].x = *x_fold - (content.points[i].x - *x_fold); 
            }
        }
    }


    // Count number of unique points 
    /*
    let mut points = HashSet::new();


    for p in content.points {
        let x = p.x;
        let y = p.y;

        points.insert((x + y) * (x + y + 1) + y);
    }

    println!("{}", points.len());*/


    // Print the points 
    let mut max_x = 0;
    let mut max_y = 0;

    // Get min and max value 
    for p in &content.points {
        if p.x > max_x {
            max_x = p.x;
        }

        if p.y > max_y {
            max_y = p.y;
        }
    }

    println!("{} {}", max_x, max_y);

    // Print points 
    for y in 0..(max_y + 1) {
        'outer: for x in 0..(max_x + 1) {
            for p in &content.points {
                if p.x == x && p.y == y {
                    print!("#");
                    continue 'outer;
                }
            } 
            print!(".");
        }
        println!();
    }


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