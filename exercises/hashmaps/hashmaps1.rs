// hashmaps1.rs
//
// Il faut définir une corbeille de fruits sous la forme d'une table de hachage. La clé
// représente le nom du fruit et la valeur représente le nombre de ce fruit
// dans la corbeille. Vous devez mettre au moins trois types de fruits différents
// (par exemple, pomme, banane, mangue) dans la corbeille et le compte total
// de tous les fruits doit être d'au moins cinq.
//
// Faites-moi compiler et passer les tests !
//
// Exécutez `rustlings hint hashmaps1` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = // TODO: déclarez votre table de hachage ici.

    // Deux bananes sont déjà données :)
    basket.insert(String::from("banana"), 2);

    // TODO: Mettez plus de fruits dans votre panier ici.

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
