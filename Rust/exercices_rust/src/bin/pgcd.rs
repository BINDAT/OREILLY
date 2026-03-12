use std::io::Write; // pour writeln! pour afficher les messages d'erreur
use std::str::FromStr; // pour FromStr::from_str pour convertir les arguments en u64

fn main(){
    let mut nombres = Vec::new(); // pour stocker les nombres à traiter

    for arg in std::env::args().skip(1) { // pour ignorer le nom du programme
        nombres.push(u64::from_str(&arg) // pour convertir l'argument en u64
            .expect("Erreur d'analyse des paramètres")); // pour gérer les erreurs de conversion
    }
    if nombres.len() == 0 {
        writeln!(std::io::stderr(), "Usag : gcd NOMBRE ...").unwrap(); // pour afficher un message d'erreur si aucun nombre n'est fourni
        std::process::exit(1); // pour quitter le programme avec un code d'erreur
    }

    let mut d = nombres[0]; // pour initialiser d avec le premier nombre de la liste
    for m in &nombres[1..]{ // pour parcourir les autres nombres de la liste
        d = gcd(d, *m); // pour calculer le pgcd de d et m et stocker le résultat dans d
    }

    println!("Le plus grand commun diviseur de {:?} est {}", // pour afficher le résultat final
            nombres, d); // pour afficher les nombres traités et le pgcd calculé
}