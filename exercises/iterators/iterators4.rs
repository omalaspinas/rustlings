// iterators4.rs
//
// Exécutez `rustlings hint iterators4` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

pub fn factorial(num: u64) -> u64 {
    // Complétez cette fonction pour obtenir la factorielle de num
    // N'utilisez pas :
    // - return
    // Essayez de ne pas utiliser:
    // - des boucles de style impératif (for, while)
    // - des variables supplémentaires
    // Pour un défi supplémentaire, n'utilisez pas:
    // - la recursion
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
