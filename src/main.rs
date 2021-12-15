mod day10;


fn main() {
    let file_name = String::from("inputs/10.txt");

    match day10::main(file_name) {
        Ok(()) => println!("Finished!"),
        Err(error) => panic!("Failed to complete problem: {:?}", error),
    }
}
