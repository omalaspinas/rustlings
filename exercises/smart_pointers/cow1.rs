// cow1.rs
//
// Cet exercice explore le type Cow, ou Clone-On-Write.
// Il peut contenir et fournir un accès immutable aux
// données empruntées, et cloner les données "paresseusement" (lazily) lorsque la mutabilité ou la propriété est
// nécessaires. Le type est conçu pour fonctionner avec des données empruntées générales via le trait
// le trait Borrow.
//
// Cet exercice a pour but de vous montrer ce à quoi il faut s'attendre lorsqu'on passe des données à Cow.
// Corrigez les tests unitaires en vérifiant Cow::Owned(_) et Cow::Borrowed(_) aux
// marqueurs TODO.
//
// Exécutez `rustlings hint cow1` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clone dans un vecteur s'il n'est pas déjà possédé.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Le clone se produit parce que `input` doit être muté.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // Il n'y a pas de clone car `input` n'a pas besoin d'être muté.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            // TODO
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // Nous pouvons aussi passer `slice` sans `&` pour que le Cow le possède directement. Dans ce
        // cas, il n'y a pas de mutation et donc pas de clone, mais le résultat est
        // toujours possédé car il n'a jamais été emprunté ou muté.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Bien entendu, c'est également le cas si une mutation se produit.
        // Dans ce cas, l'appel à `to_mut()` renvoie une
        // référence aux mêmes données qu'auparavant.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
        }
    }
}
