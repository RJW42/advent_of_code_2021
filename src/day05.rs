use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use std::cmp;


struct Line {
    a: i64,
    b: i64,
    c: i64,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32
}


pub fn main(file_name: String) -> std::io::Result<()> {
    // Read the lines 
    let lines = read_lines(&file_name)?;

    // Keep Track Of all overlapping points  
    let mut points: HashSet<[i32; 2]> = HashSet::new();


    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            let l1 = &lines[i];
            let l2 = &lines[j];

            if (l1.a * l2.b) == (l2.a * l1.b) {
                // Parallel Lines Need To Do Something Diffirent

                // Check if these lines are eqivlient 
                // use fact that (u.v)*u == (u.u)*v (if u = scalar * v)
                let l1_dot_l2 = l1.a * l2.a + l1.b * l2.b + l1.c * l2.c;
                let l1_dot_l1 = l1.a.pow(2) + l1.b.pow(2) + l1.c.pow(2);

                let a1 = l1.a * l1_dot_l2;
                let a2 = l2.a * l1_dot_l1;
                let b1 = l1.b * l1_dot_l2;
                let b2 = l2.b * l1_dot_l1;
                let c1 = l1.c * l1_dot_l2;
                let c2 = l2.c * l1_dot_l1;

                if a1 != a2 || b1 != b2 || c1 != c2 {
                    continue;
                }

                // Get the intersect of the domains of l1 and l2 
                let x_min = cmp::max(l1.x_min, l2.x_min);
                let x_max = cmp::min(l1.x_max, l2.x_max);
                let y_min = cmp::max(l1.y_min, l2.y_min);
                let y_max = cmp::min(l1.y_max, l2.y_max);

                for x in x_min..(x_max + 1){
                    for y in y_min..(y_max + 1){
                        if on_line(&l1, x as f64, y as f64) && on_line(&l2, x as f64, y as f64){
                            points.insert([x as i32, y as i32]);
                            println!("{} {}", x, y);
                        }
                    }
                }
            } else {
                // Not Parallel Calc point of intersection 
                let denominator = ((l1.a * l2.b) - (l2.a * l1.b)) as f64;

                let x = ((l1.b * l2.c) - (l2.b * l1.c)) as f64 / denominator;
                let y = ((l1.c * l2.a) - (l2.c * l1.a)) as f64 / denominator;

                if !(on_line(&l1, x, y) && on_line(&l2, x, y)){
                    continue;
                }

                points.insert([x as i32, y as i32]);
                println!("{} {}", x, y);
            }
        }
    }


    // Finished 
    println!("{}", points.len());

    Ok(())
}


fn print_line(l: &Line){
    println!(
        "{}x + {}y + {}c = 0; x:[{}, {}], y:[{} {}]",
        l.a, l.b, l.c, l.x_min, l.x_max, l.y_min, l.y_max);
}


fn on_line(l: &Line, x: f64, y: f64) -> bool{
    ((x * (l.a as f64) + y * (l.b as f64) + l.c as f64) == 0.0) && 
    x >= (l.x_min as f64) && x <= (l.x_max as f64) && 
    y >= (l.y_min as f64) && y <= (l.y_max as f64)
}


fn read_lines(file_name: &String) -> std::io::Result<Vec<Line>> {
    // Read file 
    let file = File::open(file_name)?;
    let br = BufReader::new(file);

    let mut lines: Vec<Line> = Vec::new();

    // Parse Lines 
    for line in br.lines() {
        // Get the two points 
        let points = line.unwrap();
        let points: Vec<&str> = points.split(" -> ").collect();

        let p1 = parse_point(&points[0]);
        let p2 = parse_point(&points[1]);


        // Use these points to calculate the equation of line 
        let a = p1[1] - p2[1];
        let b = p2[0] - p1[0];
        let c = (p1[0] * p2[1]) - (p2[0] * p1[1]);

        let x_min = (p1[0] < p2[0]) as i32 * p1[0] + (p1[0] >= p2[0]) as i32 * p2[0];
        let x_max = (p1[0] < p2[0]) as i32 * p2[0] + (p1[0] >= p2[0]) as i32 * p1[0];
        let y_min = (p1[1] < p2[1]) as i32 * p1[1] + (p1[1] >= p2[1]) as i32 * p2[1];
        let y_max = (p1[1] < p2[1]) as i32 * p2[1] + (p1[1] >= p2[1]) as i32 * p1[1];


        // Create line 
        let line = Line {
            a: a as i64,
            b: b as i64,
            c: c as i64,
            x_min: x_min,
            x_max: x_max,
            y_min: y_min,
            y_max: y_max
        };


        print!("{}, {} -> {}, {} => ", p1[0], p1[1], p2[0], p2[1]);
        print_line(&line);
        println!();

        lines.push(line);
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