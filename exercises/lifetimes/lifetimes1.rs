// lifetimes1.rs
//
// Le compilateur Rust doit savoir comment vérifier si les références fournies sont
// valides, afin d'indiquer au programmeur·se si une référence risque de
// de sortir de la portée avant qu'elle ne soit utilisée.
// Rappelez-vous que les références sont des emprunts et ne
// possèdent pas leurs propres données. Que se passe-t-il si leur propriétaire
// sort de la portée ?
//
// Exécutez `rustlings hint lifetimes1` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
