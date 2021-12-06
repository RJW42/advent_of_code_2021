mod day02;


fn main() {
    let file_name = String::from("inputs/02.txt");

    match day02::main(file_name) {
        Ok(()) => println!("Finished!"),
        Err(error) => panic!("Failed to complete problem: {:?}", error),
    }
}
