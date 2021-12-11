mod day08;


fn main() {
    let file_name = String::from("inputs/08.txt");

    match day08::main(file_name) {
        Ok(()) => println!("Finished!"),
        Err(error) => panic!("Failed to complete problem: {:?}", error),
    }
}
