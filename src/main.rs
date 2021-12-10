mod day06;


fn main() {
    let file_name = String::from("inputs/06.txt");

    match day06::main(file_name) {
        Ok(()) => println!("Finished!"),
        Err(error) => panic!("Failed to complete problem: {:?}", error),
    }
}
