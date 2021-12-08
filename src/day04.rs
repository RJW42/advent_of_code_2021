use std::fs::File;
use std::io::{BufReader, BufRead};


struct Board {
    store: [i32; 25],
}


pub fn main(file_name: String) -> std::io::Result<()> {
    // Read the boards 
    let mut boards = read_boards(&file_name)?;
    let moves = read_moves(&file_name)?;

    // Find winning board 
    for mov in moves{
        // Apply 
        for board in &mut boards {
            apply_move(board, mov);

            // Check for bingo 
            if contains_bingo(&board) {
                // Print winner 
                print_board(&board);

                // Print score 
                print_score(&board, mov);

                // Set marker 
                board.store[0] = -2;
            }
        }
    }

    // Finished 
    Ok(())
}


fn print_score(b: &Board, m: u32){
    let mut score = 0;

    for i in 0..25 {
        if b.store[i] != -1 {
            score += b.store[i];
        }
    }

    score = score * (m as i32);

    println!("{}", score);
}


fn print_board(b: &Board) {
    for i in 0..5 {
        for j in 0..5 {
            let num = b.store[j + (5 * i)];

            if num < 10 {
                print!("{}  ", num);
            } else {
                print!("{} ", num);
            }
        }
        println!("");
    }
    println!("");
}


fn apply_move(b: &mut Board, mov: u32){
    if b.store[0] == -2 {
        return ();
    }

    for i in 0..25 {
        if b.store[i] == (mov as i32) {
            b.store[i] = -1;
        }
    }
}


fn contains_bingo(b: &Board) -> bool{
    if b.store[0] == -2 {
        return false;
    }

    // Check Horizontal
    'outer1: for i in 0..5 {
        for j in 0..5 {
            if b.store[j + (5 * i)] != -1 {
                continue 'outer1;
            }
        }

        return true;
    }

    // Check Vertical 
    'outer2: for i in 0..5 {
        for j in 0..5 {
            if b.store[i + (5 * j)] != -1 {
                continue 'outer2;
            }
        }

        return true;
    }

    // Check Diagonal 
    /*
    'outer3: for i in 0..5 {
        if b.store[i + (5 * i)] != -1 {
            break 'outer3;
        }

        if i == 4 {
            return true;
        }
    }

    'outer4: for i in 0..5 {
        if b.store[(4 - i) + (5 * i)] != -1 {
            break 'outer4;
        }

        if i == 4 {
            return true;
        }
    } */

    false
}


fn read_moves(file_name: &String) -> std::io::Result<Vec<u32>> {
    // Read file 
    let file = File::open(file_name)?;
    let mut br = BufReader::new(file);

    let mut moves: Vec<u32> = Vec::new();
    let mut line = String::new();

    // Read line 
    br.read_line(&mut line)?;

    // Parse moves 
    for mov in line.split(',') {
        moves.push(mov.trim().parse().unwrap());
    }

    Ok(moves)
}


fn read_boards(file_name: &String) -> std::io::Result<Vec<Board>> {
    // Read file 
    let file = File::open(file_name)?;
    let br = BufReader::new(file);

    let mut boards: Vec<Board> = Vec::new();
    let mut line_count = 0;
    let mut curr_board: Vec<i32> = Vec::new();


    // Parse Boards 
    for line in br.lines().skip(1) {
        // First Line Should be White space 
        if line_count == 0 {
            line_count += 1;
            continue;
        }

        // Read each num in the line 
        let mut curr_num: u32 = 0;
        let mut size: u32 = 1;
        let mut new_num = false;

        for c in line?.chars(){
            // Check if num finished 
            if c == ' ' {
                if new_num {
                    curr_board.push(curr_num as i32);
                    curr_num = 0;
                    size = 1;
                    new_num = false;
                }
                continue;
            }

            // Coninue to parse num        
            curr_num = (curr_num * size) + c.to_digit(10).expect("Thats no number");
            size = size * 10;
            new_num = true;
        }

        // Add tail num
        curr_board.push(curr_num as i32);


        line_count += 1;

        // Check if board curent board finished 
        if line_count == 6 {
            line_count = 0;
            let mut board_arr: [i32; 25] = [0; 25];
            let mut i = 0;

            for num in curr_board{
                board_arr[i] = num;
                i += 1;
            }

            boards.push(Board {
                store: board_arr
            });

            curr_board = Vec::new();
        }
    }

    Ok(boards)
}