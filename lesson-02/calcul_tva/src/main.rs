fn main() {
    let prix_ht: f64 = 100.0;
    let taux_tva: f64 = 20.0;

    let montant_tva = prix_ht * (taux_tva / 100.0);
    let prix_ttc = prix_ht + montant_tva;

    println!("Prix HT : {:.2}€", prix_ht);
    println!("TVA ({:.0}%) : {:.2}€", taux_tva, montant_tva);
    println!("Prix TTC : {:.2}€", prix_ttc);
}
