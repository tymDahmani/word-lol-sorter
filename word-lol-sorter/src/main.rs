use std::io;

fn main() {
    println!("choose the sorting method (1: alphabet, 2: reversed)");

    let mut useless = String::new();
    io::stdin().read_line(&mut useless).expect("error");

    if useless.trim() == "1" {
        println!("enter a list of words or letters: (seperate them with spaces)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error");

        let mut words: Vec<&str> = input.trim().split_whitespace().collect();

        words.sort();

        println!("{:?}", words);
    } else if useless.trim() == "2" {
        println!("enter a list of words or letters: (seperate them with spaces)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error");

        let mut words: Vec<&str> = input.trim().split_whitespace().collect();

        words.sort_by(|a, b| b.cmp(a));

        println!("{:?}", words);
    } else {
        println!(
            "error. you inputted {} and that's not an option. retry.",
            useless
        );
    }
}
