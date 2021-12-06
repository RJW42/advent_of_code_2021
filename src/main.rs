mod day04;


fn main() {
    let file_name = String::from("inputs/04.txt");

    match day04::main(file_name) {
        Ok(()) => println!("Finished!"),
        Err(error) => panic!("Failed to complete problem: {:?}", error),
    }
}
