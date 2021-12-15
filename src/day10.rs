use std::fs::File;
use std::io::{BufReader, BufRead};

const OPEN: [char; 4] = ['(', '[', '{', '<'];
const CLOSE: [char; 4] = [')', ']', '}', '>'];
const SCORE: [u32; 4] = [3, 57, 1197, 25137];
const SCOREP2: [u32; 4] = [1, 2, 3, 4];


pub fn main(file_name: String) -> std::io::Result<()> {
    // Read in each line of the store 
    let mut result = 0;
    let mut p2_scores: Vec<u64> = Vec::new();
    let file = File::open(file_name)?;
    let br = BufReader::new(file);

    for line in br.lines(){
        let line = line?;
        let mut line_stack: Vec<char> = Vec::new();

        // Parse Line for errors 
        'outer: for c in line.chars() {
            // Open Char 
            for i in 0..4 {
                if OPEN[i] == c {
                    line_stack.push(CLOSE[i]);
                    continue 'outer;
                }
            }

            // Close Char check valid 
            let expected = line_stack.pop().unwrap();

            if c != expected {
                println!("Expected {}, but found {} instead", expected, c);

                for i in 0..4{
                    if CLOSE[i] == c {
                        result += SCORE[i];
                        break;
                    }
                }

                line_stack.clear();
                break;
            }
        }

        if !line_stack.is_empty() {
            // Build the needed string to complete 
            let mut s = String::new();
            let mut line_score = 0;

            for c in line_stack.iter().rev() {
                for i in 0..4 {
                    if CLOSE[i] == *c {
                        line_score = (line_score * 5) + (SCOREP2[i] as u64);
                        break;
                    }
                }

                s.push(*c);
            }   

            println!("Incomplete line: {} - Complete by adding {} - Score: {}", line, s, line_score);
            p2_scores.push(line_score);
        }
    }

    println!("result: {}", result);

    // Find middle p2 score 
    p2_scores.sort();


    println!("result 2: {}", p2_scores[p2_scores.len() / 2]);

    // Finished 
    Ok(())
}