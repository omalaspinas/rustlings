// from_into.rs
//
// Le trait From est utilisé pour les conversions de valeur à valeur. Si From est implémenté
// correctement pour un type, le trait Into devrait fonctionner dans l'autre sens. Vous pouvez lire
// plus d'informations à ce sujet à l'adresse https://doc.rust-lang.org/std/convert/trait.From.html
//
// Exécutez `rustlings hint from_into` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// Nous implémentons le trait Default pour l'utiliser comme solution de repli
// lorsque la chaîne fournie n'est pas convertible en un objet Person
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Votre tâche est de compléter cette implémentation pour que la ligne
// `let p = Person::from("Mark,20")` soit compilée.
// Notez que vous devrez faire le parsing de age en `usize` avec quelque chose
// comme `"4".parse::<usize>()`.
// Le résultat de ceci doit être traité de manière appropriée.
//
// Étapes :
// 1. Si la longueur de la chaîne fournie est égale à 0, alors il faut retourner la valeur par défaut de
// Person.
// 2. Diviser la chaîne donnée en fonction des virgules qu'elle contient
// 3. Extraire le premier élément de l'opération de division et l'utiliser comme nom.
// 4. Si le nom est vide, la valeur par défaut de Person est retournée.
// 5. Extraire l'autre élément de l'opération de division, parsez le en `usize`, et utilisez le comme âge.
// Si lors du parsing de l'âge, quelque chose ne va pas, alors retournez la valeur par défaut de Person
// Sinon, retournez un objet Person instancié avec les résultats.

// J'AI PAS FINI

impl From<&str> for Person {
    fn from(s: &str) -> Person {}
}

fn main() {
    // Utilisez la fonction `from`
    let p1 = Person::from("Mark,20");
    // Puisque From est implémenté pour Person, nous devrions pouvoir utiliser Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Teste que la personne par défaut est John, âgé de 30 ans
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Teste que John est retourné lorsqu'une mauvaise chaîne est fournie
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Teste que "Mark,20" fonctionne
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Teste que "Mark,twenty" renvoie la personne par défaut en raison d'une
        // erreur dans l'analyse de l'âge
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "Mike");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "Mike");
        assert_eq!(p.age, 32);
    }
}
