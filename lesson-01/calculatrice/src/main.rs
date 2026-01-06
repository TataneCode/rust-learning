use std::io;

fn main() {
    println!("=== Calculatrice Simple ===");
    println!("Entrez deux nombres et une opération (+, -, *, /, %, ^)");

    // Lire le premier nombre
    println!("\nPremier nombre :");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Erreur de lecture");
    let num1: f64 = input1.trim().parse().expect("Nombre invalide");

    // Lire l'opération
    println!("Opération (+, -, *, /, %, ^) :");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Erreur de lecture");
    let operation = operation.trim();

    // Lire le deuxième nombre
    println!("Deuxième nombre :");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Erreur de lecture");
    let num2: f64 = input2.trim().parse().expect("Nombre invalide");

    // Calculer le résultat
    let resultat = match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Erreur : Division par zéro !");
                return;
            }
        }
        "%" => num1 % num2,
        "^" => num1.powf(num2),
        _ => {
            println!("Opération invalide !");
            return;
        }
    };

    println!(
        "\nRésultat : {} {} {} = {}",
        num1, operation, num2, resultat
    );
}

