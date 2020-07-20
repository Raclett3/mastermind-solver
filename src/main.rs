use std::io::stdin;

fn prompt_usize(prompt: &str) -> usize {
    loop {
        let mut buffer = String::new();
        println!("{}", prompt);
        stdin().read_line(&mut buffer).unwrap();
        let length = buffer.len();
        if let Ok(colors) = buffer[0..length - 1].parse::<usize>() {
            return colors;
        }
    }
}

fn main() {
    println!("Mastermind solver");
    let colors = prompt_usize("How many kinds of colors does answer have?:");
    let answer_len = prompt_usize("How many colors does answer have?:");
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
    if answer_len > colors && !repetition {
        eprintln!("Length of the answer should be larger than kinds of colors if the answer has no color repetition");
        std::process::exit(1);
    }
    println!("{}", colors);
    println!("{}", answer_len);
    println!("{}", repetition);
}
