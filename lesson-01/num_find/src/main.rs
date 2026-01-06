use rand::random_range;
use std::io;

fn main() {
    println!("=== *** Num Finder GAME *** ===");
    println!("Find the number between 1 and 10 000 !");

    let joker_number = random_range(1..10000);
    let mut win = false;

    // Lire le premier nombre
    for index in 1..11 {
        println!("\nEnter number try {}", index);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");
        let user_num: i32 = input.trim().parse().expect("Invalid number...");
        if joker_number < user_num {
            println!("\nTry lower !");
        }
        if joker_number > user_num {
            println!("Try higher !");
        }
        if joker_number == user_num {
            println!("You win \\o/!");
            win = true;
            break;
        }
    }
    if !win {
        println!("You loose :( !");
        println!("Number was : {}", joker_number);
    }
}

