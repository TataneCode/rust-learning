fn contient_majuscule(mot_de_passe: &str) -> bool {
    mot_de_passe.chars().any(|c| c.is_uppercase())
}

fn contient_minuscule(mot_de_passe: &str) -> bool {
    mot_de_passe.chars().any(|c| c.is_lowercase())
}

fn contient_chiffre(mot_de_passe: &str) -> bool {
    mot_de_passe.chars().any(|c| c.is_numeric())
}

fn est_assez_long(mot_de_passe: &str) -> bool {
    mot_de_passe.len() >= 8
}

fn valider_mot_de_passe(mot_de_passe: &str) -> bool {
    let mut criteres_valides = 0;
    let mut messages = Vec::new();

    if est_assez_long(mot_de_passe) {
        criteres_valides += 1;
    } else {
        messages.push("❌ Au moins 8 caractères");
    }

    if contient_majuscule(mot_de_passe) {
        criteres_valides += 1;
    } else {
        messages.push("❌ Au moins une majuscule");
    }

    if contient_minuscule(mot_de_passe) {
        criteres_valides += 1;
    } else {
        messages.push("❌ Au moins une minuscule");
    }

    if contient_chiffre(mot_de_passe) {
        criteres_valides += 1;
    } else {
        messages.push("❌ Au moins un chiffre");
    }

    println!("\nAnalyse du mot de passe :");
    println!("Critères validés : {}/4", criteres_valides);

    for message in messages {
        println!("{}", message);
    }

    criteres_valides == 4
}

fn main() {
    use std::io;

    println!("=== VALIDATEUR DE MOT DE PASSE ===");
    println!("Critères :");
    println!("- Au moins 8 caractères");
    println!("- Au moins une majuscule");
    println!("- Au moins une minuscule");
    println!("- Au moins un chiffre");

    loop {
        println!("\nEntrez un mot de passe (ou 'quit' pour quitter) :");
        let mut mot_de_passe = String::new();
        io::stdin()
            .read_line(&mut mot_de_passe)
            .expect("Erreur de lecture");
        let mot_de_passe = mot_de_passe.trim();

        if mot_de_passe == "quit" {
            break;
        }

        if valider_mot_de_passe(mot_de_passe) {
            println!("✅ Mot de passe VALIDE !");
        } else {
            println!("❌ Mot de passe INVALIDE");
        }
    }
}
