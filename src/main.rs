mod day03;


fn main() {
    let file_name = String::from("inputs/03.txt");

    match day03::main(file_name) {
        Ok(()) => println!("Finished!"),
        Err(error) => panic!("Failed to complete problem: {:?}", error),
    }
}
