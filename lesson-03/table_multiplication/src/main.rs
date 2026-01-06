use std::io;

fn generate_multiplication_table(number: u32) {
    for multiplicateur in 1..=10 {
        println!(
            "{} que multiplie {} égal {}",
            number,
            multiplicateur,
            number * multiplicateur
        );
    }
}

fn lire_nombre(message: &str) -> u32 {
    println!("{}", message);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur !");

    input
        .trim()
        .parse()
        .expect("Entier non-signé sur 32 bits attendu !")
}

fn main() {
    println!("*** TABLES DE MULTIPLICATION ***");
    loop {
        let nombre = lire_nombre("Entrez un nombre (0 pour quitter) :");
        match nombre {
            0 => {
                println!("A plus tard, Gaspard !");
                break;
            }
            _ => generate_multiplication_table(nombre),
        }
    }
}
