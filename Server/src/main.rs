use std::io::{self, Write};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    loop {
        let mut input = String::new();
        print!("Enter the name to greet (or 'exit' to quit): ");
        io::stdout().flush().unwrap(); // Flush to ensure the prompt prints

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let args = Args {
            name: input.to_string(),
            count: 1, // You can modify this part to allow input for count
        };

        for _ in 0..args.count {
            println!("Hello, {}!", args.name);
        }
    }
}
