extern crate iron; // externe crate pour le serveur web
#[macro_use] extern crate mime; // macro pour les types MIME

use iron::prelude::*; // prélude pour les types de base d'Iron
use iron::status; // pour les codes de statut HTTP

fn main() {
    println!("Serveur en http://localhost:3000..."); // message pour indiquer que le serveur est en cours d'exécution
    Iron::new(get_form).http("localhost:3000").unwrap(); // création d'un nouveau serveur Iron qui écoute sur localhost:3000 et utilise la fonction get_form pour gérer les requêtes
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new(); // création d'une nouvelle réponse HTTP

    response.set_mut(status::Ok); // définition du statut de la réponse à 200 OK
    response.set_mut(mime!(Text/Html; Charset=Utf8)); // définition du type de contenu de la réponse à text/html avec encodage UTF-8
    response.set_mut(r#"
        <title>Calculatrice de PGCD</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Calculer le PGCD</button>
        </form>
        "#); // définition du corps de la réponse avec un formulaire HTML pour calculer le PGCD de deux nombres

        Ok(response) // retour de la réponse HTTP
}
extern crate iron; // externe crate pour le serveur web
use router::Router; // pour le routage des requêtes