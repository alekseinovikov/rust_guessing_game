use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Please input your guess.");
        let guess: u32 = read_and_parse_line();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

fn read_and_parse_line() -> u32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    return match input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("You typed not a number! Try one more time, please!");
            return read_and_parse_line()
        },
    }
}
