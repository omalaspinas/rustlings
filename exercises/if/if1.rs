// if1.rs
//
// Exécutez `rustlings hint if1` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complétez cette fonction pour qu'elle renvoie le plus grand nombre !
    // Ne pas utiliser :
    // - un autre appel de fonction
    // - des variables supplémentaires
}

// Ne vous préoccupez pas de ce qu'il y a en dessous pour l'instant :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
