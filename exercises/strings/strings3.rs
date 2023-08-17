// strings3.rs
//
// Exécutez `rustlings hint strings3` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    // TODO: Supprimez les espaces aux deux extrémités d'une chaîne de caractères !
    ???
}

fn compose_me(input: &str) -> String {
    // TODO: Ajoutez " world !" à la chaîne de caractères! Il y a plusieurs façons de le faire!
    ???
}

fn replace_me(input: &str) -> String {
    // TODO: Remplacez "cars" par "balloons" dans la chaîne de caractères !
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
