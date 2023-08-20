// box1.rs
//
// Au moment de la compilation, Rust a besoin de connaître la place occupée par un type. Cela
// devient problématique pour les types récursifs, où une valeur peut avoir comme partie
// une autre valeur du même type. Pour contourner le problème, nous pouvons utiliser un
// `Box` - un pointeur intelligent utilisé pour stocker des données sur le tas, qui nous permet également
// d'envelopper un type récursif.
//
// Le type récursif que nous implémentons dans cet exercice est la ``liste de cons`` - une
// structure de données fréquemment utilisée dans les langages de programmation fonctionnels. Chaque
// élément d'une liste cons contient deux éléments : la valeur de l'élément courant et
// l'élément suivant. Le dernier élément est une valeur appelée `Nil`.
//
// Étape 1 : utiliser un `Box` dans la définition de l'enum pour compiler le code
// Etape 2 : créer des listes de cons vides et non vides en remplaçant `todo!()` par
// des listes de cons vides et non vides
//
// Note : les tests ne doivent pas être modifiés
//
// Exécutez `rustlings hint box1` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    todo!()
}

pub fn create_non_empty_list() -> List {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
