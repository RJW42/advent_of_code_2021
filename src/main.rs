mod day09;


fn main() {
    let file_name = String::from("inputs/09.txt");

    match day09::main(file_name) {
        Ok(()) => println!("Finished!"),
        Err(error) => panic!("Failed to complete problem: {:?}", error),
    }
}
