mod day01;


fn main() {
    let file_name = String::from("inputs/01.txt");

    match day01::main(file_name) {
        Ok(()) => println!("Finished!"),
        Err(error) => panic!("Failed to complete problem: {:?}", error),
    }
}
