use std::io::{stdin, stdout, Write};

fn main() {

    let mut s=String::new();
    
    println!("Made by Froxcey cuz why not?");
    println!("Available under GPL-V3, check <https://www.gnu.org/licenses/> for additional info.");
    println!("Who asked?");
    print!(">");
    stdout().flush().unwrap();
    stdin().read_line(&mut s).expect("Tf did you said?");

    match s.to_lowercase().trim() {
        "" | "no one" => println!("Exactly!"),
        "me" => println!("No? Go suck yourself."),
        "your mom" => println!("Your face."),
        _ => println!("Yes {}, you stupid!", s.trim())
    }

}
