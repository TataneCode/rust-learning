use std::io;

fn addition(a: f64, b: f64) -> f64 {
    a + b
}

fn soustraction(a: f64, b: f64) -> f64 {
    a - b
}

fn multiplication(a: f64, b: f64) -> f64 {
    a * b
}

fn division(a: f64, b: f64) -> f64 {
    if b != 0.0 {
        a / b
    } else {
        println!("Erreur : division par zéro !");
        0.0
    }
}

fn lire_nombre(message: &str) -> f64 {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");
    input
        .trim()
        .parse()
        .expect("Veuillez entrer un nombre valide")
}

fn afficher_menu() {
    println!("\n=== CALCULATRICE ===");
    println!("1. Addition");
    println!("2. Soustraction");
    println!("3. Multiplication");
    println!("4. Division");
    println!("5. Quitter");
    println!("====================");
}

fn main() {
    loop {
        afficher_menu();

        let choix = lire_nombre("Votre choix :") as i32;

        if choix == 5 {
            println!("Au revoir !");
            break;
        }

        if choix < 1 || choix > 5 {
            println!("Choix invalide !");
            continue;
        }

        let a = lire_nombre("Premier nombre :");
        let b = lire_nombre("Deuxième nombre :");

        let resultat = match choix {
            1 => addition(a, b),
            2 => soustraction(a, b),
            3 => multiplication(a, b),
            4 => division(a, b),
            _ => 0.0,
        };

        let operation = match choix {
            1 => "+",
            2 => "-",
            3 => "×",
            4 => "÷",
            _ => "?",
        };

        println!("\nRésultat : {} {} {} = {}", a, operation, b, resultat);
    }
}
