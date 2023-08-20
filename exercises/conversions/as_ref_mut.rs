// as_ref_mut.rs
//
// AsRef et AsMut permettent des conversions de référence à référence peu coûteuses. Pour en savoir plus
// à leur sujet sur https://doc.rust-lang.org/std/convert/trait.AsRef.html et
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectivement.
//
// Exécutez `rustlings hint as_ref_mut` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

// Obtient le nombre d'octets (et non de caractères) dans l'argument donné.
// TODO : Ajoutez le trait AsRef de manière appropriée en tant que contrainte de trait.
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtient le nombre de caractères (et non d'octets) dans l'argument donné.
// TODO : Ajoutez le trait AsRef de manière appropriée en tant que contrainte de trait.
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Élève un nombre au carré en utilisant as_mut().
// TODO : Ajouter la contrainte de trait appropriée.
fn num_sq<T>(arg: &mut T) {
    // TODO: Implémentez le corps de la fonction.
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
