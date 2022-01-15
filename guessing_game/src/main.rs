use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the Number");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Secret Number {:?} ", secret_number);


    println!("Please input your guess");
    
    loop {
    let mut _guess = String::new();

    io::stdin().read_line(&mut _guess).expect("Failed to read");

    let _guess: u32 = match _guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,  
    };

    println!("You guessed : {:?}", _guess);
    
    //let result = 1.cmp(&2);
    match _guess.cmp(&secret_number) {
        Ordering::Less => println!("{}","Small".red()),
        Ordering::Equal => {println!("{}","Equal".green());
        break;
    }
        Ordering::Greater => println!("{}","Greater".red()),
        
    }
    }   
    }

