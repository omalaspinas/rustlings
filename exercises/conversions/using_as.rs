// using_as.rs
//
// La conversion des types en Rust se fait via l'utilisation de l'opérateur `as`. Veuillez noter
// que l'opérateur `as` n'est pas seulement utilisé pour la conversion de types. Il aide aussi à
// renommer les imports.
//
// Le but est de s'assurer que la division n'échoue pas à la compilation et qu'elle
// retourne le bon type.
//
// Exécutez `rustlings hint using_as` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
