use std::io::stdin;

fn main() {
    println!("Mastermind solver");
    let colors = loop {
        let mut buffer = String::new();
        println!("How many colors will you use:");
        stdin().read_line(&mut buffer).unwrap();
        let length = buffer.len();
        if let Ok(colors) = buffer[0..length - 1].parse::<usize>() {
            break colors;
        }
    };
    let repetition = loop {
        let mut buffer = String::new();
        println!("Do you allow repetition?(y/N):");
        stdin().read_line(&mut buffer).unwrap();
        break match buffer.chars().next().unwrap() {
            'Y' | 'y' => true,
            'N' | 'n' | '\n' => false,
            _ => continue,
        };
    };
    println!("{}", colors);
    println!("{}", repetition);
}
