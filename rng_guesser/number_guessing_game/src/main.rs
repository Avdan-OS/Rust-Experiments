use rand::thread_rng;
use rand::Rng;

use std::io;

fn main() {
    println!("Program Started...");

    let start_txt = "
██████╗     ███╗   ██╗     ██████╗              ██████╗ ██╗   ██╗███████╗███████╗███████╗███████╗██████╗ 
██╔══██╗    ████╗  ██║    ██╔════╝             ██╔════╝ ██║   ██║██╔════╝██╔════╝██╔════╝██╔════╝██╔══██╗
██████╔╝    ██╔██╗ ██║    ██║  ███╗            ██║  ███╗██║   ██║█████╗  ███████╗███████╗█████╗  ██████╔╝
██╔══██╗    ██║╚██╗██║    ██║   ██║            ██║   ██║██║   ██║██╔══╝  ╚════██║╚════██║██╔══╝  ██╔══██╗
██║  ██║    ██║ ╚████║    ╚██████╔╝            ╚██████╔╝╚██████╔╝███████╗███████║███████║███████╗██║  ██║
╚═╝  ╚═╝    ╚═╝  ╚═══╝     ╚═════╝              ╚═════╝  ╚═════╝ ╚══════╝╚══════╝╚══════╝╚══════╝╚═╝  ╚═╝                                                                                                                                                                                                                                         
    ";

    println!("{start_txt}");
    println!("Made by lucasodev - UNLICENSE");
    println!(" ");
    println!(" ");
    
    guessing();
}

fn guessing() {
    let target_number = get_random_number();

    loop {
        let guess = get_user_input();
        if guess == random_number {
            println!("Yeah, it was right, the number was {random_number}!");
            return;
        } else {
            println!("Sorry, Your answer was wrong!");
            continue;
        }
    }
}

fn get_random_number() -> i64 {
    let _random_number: i64 = thread_rng().gen_range(0, 11);

    _random_number
}

fn get_user_input() -> i64 {
    println!("Please input a guessing number under (Must be between 0 - 10):");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error while reading line!");

    let _int_guess: i64 = guess.trim().parse().unwrap();

    _int_guess
}
