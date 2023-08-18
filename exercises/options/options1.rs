// options1.rs
//
// Exécutez `rustlings hint options1` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.


// I AM NOT DONE

// Cette fonction renvoie la quantité de glace restante dans le réfrigérateur.
// Si on est avant 22h, il reste 5 morceaux. A 22h, quelqu'un les mange
// tous, donc il n'en restera plus :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // Nous utilisons ici le système de 24 heures, donc 10PM correspond à 22 et 12AM à
    // 0. La sortie de l'option doit traiter avec élégance les cas où
    // time_of_day > 23.
    // TODO : Complétez le corps de la fonction - n'oubliez pas de retourner une Option !
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Corrigez ce test. Comment accéder à la valeur contenue dans l'Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, 5);
    }
}
