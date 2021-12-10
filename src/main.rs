mod day07;


fn main() {
    let file_name = String::from("inputs/07.txt");

    match day07::main(file_name) {
        Ok(()) => println!("Finished!"),
        Err(error) => panic!("Failed to complete problem: {:?}", error),
    }
}
