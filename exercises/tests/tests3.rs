// tests3.rs
//
// Ce test ne teste pas notre fonction -- faites en sorte qu'il le fasse de manière à ce que
// le test passe. Ensuite, écrivez un second test qui teste si nous obtenons le
// résultat que nous attendons lorsque nous appelons `is_even(5)` (est_pair).
//
// Exécutez `rustlings hint tests3` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!();
    }

    #[test]
    fn is_false_when_odd() {
        assert!();
    }
}
