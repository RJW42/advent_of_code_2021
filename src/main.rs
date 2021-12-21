mod day13;


fn main() {
    let file_name = String::from("inputs/13.txt");

    match day13::main(file_name) {
        Ok(()) => println!("Finished!"),
        Err(error) => panic!("Failed to complete problem: {:?}", error),
    }
}
