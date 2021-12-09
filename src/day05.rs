use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;


struct Line {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32, 
    m: f64
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

            if l1.m != l2.m {
                // Get Overlapping Point 
                let mut p = [l2.x1, l1.y1];
                
                if l1.m == 90.0 {
                    p = [l1.x1, l2.y1];
                }

                if p[0] >= l1.x1 && p[0] >= l2.x1 && p[0] <= l1.x2 && p[0] <= l2.x2 && 
                   p[1] >= l1.y1 && p[1] >= l2.y1 && p[1] <= l1.y2 && p[1] <= l2.y2 {
                    points.insert(p);    
                }
            } else if l1.m == 90.0 && l1.x1 == l2.x1 {
                // Get Overlapping vertical Points 
                let y_size = 
                    ((l1.y2 > l2.y2) as i32 * l2.y2 + (l1.y2 <= l2.y2) as i32 * l1.y2) - 
                    ((l1.y1 > l2.y1) as i32 * l1.y1 + (l1.y1 <= l2.y1) as i32 * l2.y1) + 1;

                printline(&l1);
                printline(&l2);
                
                for y_inc in 0..y_size {
                    let y = (l1.y1 > l2.y1) as i32 * l1.y1 + (l1.y1 <= l2.y1) as i32 * l2.y1 + y_inc;

                    println!("{} {}", l1.x1, y);

                    points.insert([l1.x1, y]);
                }

            } else if l1.m == 0.0 && l1.y1 == l2.y1{
                // Get Overlapping horizontal Points 
                let x_size = 
                    ((l1.x2 > l2.x2) as i32 * l2.x2 + (l1.x2 <= l2.x2) as i32 * l1.x2) - 
                    ((l1.x1 > l2.x1) as i32 * l1.x1 + (l1.x1 <= l2.x1) as i32 * l2.x1) + 1;

                printline(&l1);
                printline(&l2);

            
                for x_inc in 0..x_size {
                    let x = (l1.x1 > l2.x1) as i32 * l1.x1 + (l1.x1 <= l2.x1) as i32 * l2.x1 + x_inc;

                    println!("{} {}", x, l1.y1);

                    points.insert([x, l1.y1]);
                }
            }

            
            /* // Todo: change this miread question. 
            let overlap_h = 
                ((l1.m == l2.m && l1.m == 90.0 && l1.x1 == l2.x1) as i32) * ( 1 +
                    ((l1.y2 > l2.y2) as i32 * l2.y2 + (l1.y2 <= l2.y2) as i32 * l1.y2) - 
                    ((l1.y1 > l2.y1) as i32 * l1.y1 + (l1.y1 <= l2.y1) as i32 * l2.y1)
                );
            
            let overlap_v = 
                ((l1.m == l2.m && l1.m == 0.0 && l1.y1 == l2.y1) as i32) * ( 1 + 
                    ((l1.x2 > l2.x2) as i32 * l2.x2 + (l1.x2 <= l2.x2) as i32 * l1.x2) - 
                    ((l1.x1 > l2.x1) as i32 * l1.x1 + (l1.x1 <= l2.x1) as i32 * l2.x1)
                );

            let overlap_cross = 
                (l1.m != l2.m && (
                    (l1.m == 90.0 && // L1: x=a, L2: y=b
                        l1.x1 >= l2.x1 && l1.x1 <= l2.x2 && 
                        l2.y1 >= l1.y1 && l2.y1 <= l1.y2) ||  
                    (l1.m == 0.0 &&  // L1: y=a, L2: x=b
                        l1.y1 >= l2.y1 && l1.y1 <= l2.y2 && 
                        l2.x1 >= l1.x1 && l2.x1 <= l1.x2)   
                )) as i32;

            
            overlapping += 
                ((overlap_v > 0) as i32) * (overlap_v) + 
                ((overlap_h > 0) as i32) * (overlap_h) + 
                overlap_cross;  */
        }
    }


    // Finished 
    println!("{}", points.len());

    Ok(())
}


fn printline(l: &Line) {
    println!("{},{} -> {},{} : {}", l.x1, l.y1, l.x2, l.y2, l.m);
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

        // Order points 
        let (p1, p2) = match (p1[0].pow(2) + p1[1].pow(2)) > (p2[0].pow(2) + p2[1].pow(2)) {
            true => (p2, p1),
            false => (p1, p2)
        };


        // Calculate the gradient 
        let deltax = p2[0] - p1[0];
        let deltay = p2[1] - p1[1];

        let m = match deltax {
            0 => 90.0,
            _ => ((deltay / deltax) as f64).atan()
        };

        //println!("{},{} -> {},{} : {}", p1[0], p1[1], p2[0], p2[1], m);

        // Check if hor or vertical 
        if m == 0.0 || m == 90.0 {
            lines.push(Line {
                x1: p1[0],
                x2: p2[0],
                y1: p1[1],
                y2: p2[1], 
                m: m
            });
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