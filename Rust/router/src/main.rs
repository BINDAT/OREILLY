fn main() {
    let mut router = Router::new(); // Crée un nouveau routeur pour gérer les différentes routes de l'application

    router.get("/", get_form, "root"); // Définit une route GET pour la racine ("/") qui affiche un formulaire HTML pour saisir les nombres
    router.post("/gcd", post_gcd, "gcd"); // Définit une route POST pour "/gcd" qui traite les données du formulaire, calcule le PGCD et affiche le résultat

    println("Serveur en http://localhost:3000..."); // Affiche un message dans la console pour indiquer que le serveur est en cours d'exécution
    Iron::new(router).http("localhost:3000").unwrap(); // Démarre le serveur HTTP sur localhost à port 3000 en utilisant le routeur défini précédemment
}