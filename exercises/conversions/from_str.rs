// from_str.rs
//
// Ceci est similaire à from_into.rs, mais cette fois nous allons implémenter `FromStr` et
// renvoyer les erreurs au lieu de revenir à la valeur par défaut. De plus, lors de
// l'implémentation de FromStr, vous pouvez utiliser la méthode `parse` sur les chaînes de caractères pour générer
// un objet du type de l'implémenteur. Vous pouvez en savoir plus à ce sujet à l'adresse suivante
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Exécutez `rustlings hint from_str` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// Nous utiliserons ce type d'erreur pour l'implémentation de `FromStr`.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Chaîne d'entrée vide
    Empty,
    // Nombre incorrect de champs
    BadLen,
    // Champ de nom vide
    NoName,
    // Erreur wrappée de parse::<usize>()
    ParseInt(ParseIntError),
}

// J'AI PAS FINI

// Étapes:
// 1. Si la longueur de la chaîne fournie est égale à 0, une erreur doit être retournée
// 2. Diviser la chaîne donnée sur les virgules présentes dans celle-ci
// 3. Seuls 2 éléments doivent être renvoyés à la suite de la division, sinon une erreur est retournée
// 4. Le premier élément est extrait de l'opération de division et utilisé comme nom
// 5. Extraire l'autre élément de l'opération de division et le parser comme un l'âge en `usize`
//    avec quelque chose comme `"4".parse::<usize>()`
// 6. Si, lors de l'extraction du nom et de l'âge, quelque chose ne va pas, une erreur
//    doit être retournée
//    Si tout se passe bien, il faut renvoyer le résultat d'un objet Personne
//
// Remarque: `Box<dyn Error>` implémente `From<&'_ str>`. Cela signifie que si
// vous voulez retourner un message d'erreur sous forme de chaîne de caractères, vous pouvez le faire en utilisant simplement
// return `Err("mon message d'erreur".into())`.

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {}
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
