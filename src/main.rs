mod day11;


fn main() {
    let file_name = String::from("inputs/11.txt");

    match day11::main(file_name) {
        Ok(()) => println!("Finished!"),
        Err(error) => panic!("Failed to complete problem: {:?}", error),
    }
}
