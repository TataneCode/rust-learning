fn trouve_plus_grand<'a>(liste: &'a [i32]) -> Option<&'a i32> {
    // LIFETIME 'a : indique que la référence retournée vivra aussi longtemps
    // que la référence d'entrée 'liste'. Cela garantit qu'on ne retourne pas
    // une référence vers des données qui n'existent plus.

    // Gestion du cas où la liste est vide
    if liste.is_empty() {
        return None; // Option permet de gérer l'absence de valeur
    }

    // On initialise avec le premier élément pour gérer les nombres négatifs
    let mut index_max = 0;

    // On parcourt à partir de l'index 1 (on a déjà pris l'index 0)
    for index in 1..liste.len() {
        // Comparaison avec la valeur actuelle du max, pas avec une constante
        if liste[index] > liste[index_max] {
            index_max = index;
        }
    }

    // RÉFÉRENCE : on retourne &liste[index_max], pas liste[index_max]
    // Le & est crucial : on retourne une référence (emprunt) vers l'élément,
    // pas une copie. Cela respecte le système d'ownership de Rust.
    Some(&liste[index_max])
}

fn filtre_pairs(liste: &[i32]) -> Vec<i32> {
    // Créer un nouveau vecteur avec uniquement les nombres pairs

    // MUTABILITÉ : on doit déclarer la variable comme 'mut' pour pouvoir
    // la modifier avec push(). Par défaut, toutes les variables sont
    // immutables en Rust pour la sécurité.
    let mut nombres_pairs: Vec<i32> = vec![];

    // Itération : 'nombre' est une référence (&i32) car on itère sur &[i32]
    for nombre in liste {
        if nombre % 2 == 0 {
            // DÉRÉFÉRENCEMENT : on utilise *nombre pour obtenir la valeur i32
            // à partir de la référence &i32. Le Vec stocke des valeurs, pas
            // des références. On pourrait aussi écrire : nombres_pairs.push(*nombre);
            nombres_pairs.push(*nombre);
        }
    }

    nombres_pairs
}

// ========== VERSIONS SIMPLIFIÉES ==========

fn trouve_plus_grand_simple(liste: &[i32]) -> Option<i32> {
    // VERSION SIMPLE : retourne la VALEUR (i32), pas une référence
    // ✅ Pas de lifetime nécessaire
    // ✅ Plus simple à utiliser
    // ✅ i32 implémente Copy, donc copier est gratuit (8 octets)

    if liste.is_empty() {
        return None;
    }

    // On copie directement la première valeur
    let mut max = liste[0];

    // Parcours du reste de la liste
    // Le pattern &nombre déstructure automatiquement la référence
    for &nombre in &liste[1..] {
        if nombre > max {
            max = nombre;
        }
    }

    // On retourne une COPIE de la valeur
    Some(max)
}

fn filtre_pairs_simple(liste: &[i32]) -> Vec<i32> {
    // VERSION SIMPLE : utilise les itérateurs (style fonctionnel)
    // ✅ Plus idiomatique en Rust
    // ✅ Pas besoin de 'mut'
    // ✅ Plus concis

    liste
        .iter()              // Crée un itérateur sur les références
        .filter(|&&n| n % 2 == 0)  // Garde seulement les pairs
        .copied()            // Copie les valeurs (déréférence automatique)
        .collect()           // Collecte dans un Vec<i32>
}

// ========== EXEMPLES AVEC STRING ==========

fn trouve_plus_long_avec_ref<'a>(mots: &'a [String]) -> Option<&'a String> {
    // ✅ AVEC RÉFÉRENCE : efficace, pas de copie
    // Le lifetime 'a garantit que la référence retournée reste valide
    // tant que le slice 'mots' existe

    if mots.is_empty() {
        return None;
    }

    let mut plus_long = &mots[0];

    for mot in &mots[1..] {
        if mot.len() > plus_long.len() {
            plus_long = mot;  // On stocke juste une référence
        }
    }

    Some(plus_long)  // ✅ Retourne une référence, pas de copie
}

fn trouve_plus_long_sans_ref(mots: &[String]) -> Option<String> {
    // ⚠️ SANS RÉFÉRENCE : fonctionne mais COÛTEUX
    // String ne peut pas être copié (pas Copy), il faut cloner

    if mots.is_empty() {
        return None;
    }

    let mut plus_long = mots[0].clone();  // ❌ Clone = allocation mémoire

    for mot in &mots[1..] {
        if mot.len() > plus_long.len() {
            plus_long = mot.clone();  // ❌ Clone à chaque itération !
        }
    }

    Some(plus_long)  // ❌ Retourne une copie complète de la String
}

fn main() {
    use std::time::Instant;

    let nombres = vec![23, 45, 12, 67, 89, 34];

    println!("=== VERSIONS AVEC RÉFÉRENCES ===");
    let start = Instant::now();
    if let Some(max) = trouve_plus_grand(&nombres) {
        println!("Plus grand : {}", max);
    }
    let duration = start.elapsed();
    println!("⏱️  Temps: {:?}", duration);

    let start = Instant::now();
    let pairs = filtre_pairs(&nombres);
    let duration = start.elapsed();
    println!("Nombres pairs : {:?}", pairs);
    println!("⏱️  Temps: {:?}", duration);

    println!("\n=== VERSIONS SIMPLIFIÉES ===");
    let start = Instant::now();
    if let Some(max) = trouve_plus_grand_simple(&nombres) {
        println!("Plus grand : {}", max);
    }
    let duration = start.elapsed();
    println!("⏱️  Temps: {:?}", duration);

    let start = Instant::now();
    let pairs_simple = filtre_pairs_simple(&nombres);
    let duration = start.elapsed();
    println!("Nombres pairs : {:?}", pairs_simple);
    println!("⏱️  Temps: {:?}", duration);

    println!("\n=== EXEMPLE AVEC STRING ===");
    let mots = vec![
        String::from("chat"),
        String::from("éléphant"),
        String::from("souris"),
        String::from("hippopotame"),
    ];

    // Avec référence : efficace
    let start = Instant::now();
    if let Some(mot) = trouve_plus_long_avec_ref(&mots) {
        println!("Plus long (avec ref) : {}", mot);
    }
    let duration = start.elapsed();
    println!("⏱️  Temps: {:?}", duration);

    // Sans référence : coûteux (clone à chaque comparaison)
    let start = Instant::now();
    if let Some(mot) = trouve_plus_long_sans_ref(&mots) {
        println!("Plus long (sans ref) : {}", mot);
    }
    let duration = start.elapsed();
    println!("⏱️  Temps: {:?}", duration);

    // On peut toujours utiliser 'mots' après
    println!("Tous les mots : {:?}", mots);

    println!("\n=== BENCHMARK AVEC GRANDES DONNÉES ===");
    // Test avec un grand vecteur pour voir la différence de performance
    let grands_nombres: Vec<i32> = (0..100000).collect();

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = trouve_plus_grand(&grands_nombres);
    }
    let duration = start.elapsed();
    println!("trouve_plus_grand (1000x) : {:?}", duration);

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = trouve_plus_grand_simple(&grands_nombres);
    }
    let duration = start.elapsed();
    println!("trouve_plus_grand_simple (1000x) : {:?}", duration);

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = filtre_pairs(&grands_nombres);
    }
    let duration = start.elapsed();
    println!("filtre_pairs (1000x) : {:?}", duration);

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = filtre_pairs_simple(&grands_nombres);
    }
    let duration = start.elapsed();
    println!("filtre_pairs_simple (1000x) : {:?}", duration);

    // Test String avec grande données
    let grands_mots: Vec<String> = (0..1000)
        .map(|i| format!("mot_{:05}", i))
        .collect();

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = trouve_plus_long_avec_ref(&grands_mots);
    }
    let duration = start.elapsed();
    println!("trouve_plus_long_avec_ref (1000x) : {:?}", duration);

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = trouve_plus_long_sans_ref(&grands_mots);
    }
    let duration = start.elapsed();
    println!("trouve_plus_long_sans_ref (1000x) : {:?}", duration);
}
