mod day05;


fn main() {
    let file_name = String::from("inputs/05.txt");

    match day05::main(file_name) {
        Ok(()) => println!("Finished!"),
        Err(error) => panic!("Failed to complete problem: {:?}", error),
    }
}
