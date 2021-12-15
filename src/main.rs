mod day12;


fn main() {
    let file_name = String::from("inputs/12.txt");

    match day12::main(file_name) {
        Ok(()) => println!("Finished!"),
        Err(error) => panic!("Failed to complete problem: {:?}", error),
    }
}
