extern crate urlencoded; //Crate pour analyser les données de formulaire

use std::str::FromStr; //Pour convertir les chaînes de caractères en nombres
use urlencoded::UrlEncodedBody; //Pour analyser les données de formulaire

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new(); //Créer une réponse vide pour y mettre le résultat ou les erreurs

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Erreur analyse formulaire: {:?}\n", e));
            return Ok(response); //Si les données de formulaire ne sont pas valides, retourner une erreur
        }
        Ok(map) => map
    }; //Analyser les données de formulaire et les mettre dans un HashMap

let unparsed_numbers = match form_data.get("n") {
    None => {
            response.set(status::BadRequest);
            response.set(format!("Pas de 'n' dans le formulaire\n"));
            return Ok(response); //Si le champ 'n' n'est pas présent dans le formulaire, retourner une erreur
        }
        Some(nums) => nums
    }; //Récupérer les valeurs associées à la clé 'n' dans le formulaire, qui sont les nombres dont on veut calculer le PGCD

let mut nombres = Vec::new(); //Créer un vecteur pour stocker les nombres convertis en u64
for unparsed in unparsed_numbers {
    match u64::from_str(&unparsed) {
        Err(_) => {
            response.set_mut(status::BadRequest);
            response.set_mut(
                format!("Valeur non num. pour 'n: {:?}\n",
                            unparsed));
            return Ok(response); //Si une des valeurs associées à 'n' n'est pas un nombre valide, retourner une erreur
            }
            Ok(n) => { nombres.push(n); } //Si la conversion est réussie, ajouter le nombre au vecteur de nombres
        }
    }    


    let mut d = nombre[0]; //Initialiser d avec le premier nombre du vecteur
    for m in &nombres[1..]{ //Pour chaque nombre à partir du deuxième dans le vecteur
    d = gcd(d, *m); //Calculer le PGCD de d et du nombre courant m, et stocker le résultat dans d
} //Calculer le PGCD de tous les nombres du vecteur en utilisant la fonction gcd

    response.set_mut(status::Ok); //Définir le statut de la réponse à OK pour indiquer que le calcul a réussi
    response.set_mut(mime!(Text/Html; Charset=Utf8)); //Définir le type de contenu de la réponse à HTML avec encodage UTF-8 pour pouvoir afficher le résultat correctement dans un navigateur
    response.set_mut(
        format!("Le PGCD de {:?} est <b>{}≤/b>\n", 
        nombres, d)); //Définir le corps de la réponse avec un message indiquant les nombres dont on a calculé le PGCD et le résultat du PGCD, en utilisant une balise HTML <b> pour mettre le résultat en gras
    Ok(response) //Retourner la réponse avec le résultat du PGCD ou les erreurs éventuelles
}