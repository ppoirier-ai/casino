use std::io;
use slotmachine::spin;

fn main() {

    println!("Welcome to our Crypto Slot Machine!!!");

    loop {
        println!("Type what you would like to do next? [1] for Spin, [2] to stop, [3] to cash out, [4] to deposit");

        let mut selection = String::new();

        io::stdin()
        .read_line(&mut selection)
        .expect("Please input a single number");

        let selection: u32 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You chose: {selection}");

        match selection {
            1 => slotmachine::spin(),
            3 => { println!("You selected to cash out"); },
            4 => { println!("You selected to deposit"); },
            2 => { println!("Good bye and see you soon!"); break; },
            _ => { println!("Pick a number between 1 and 4"); }
        }
    }
}
