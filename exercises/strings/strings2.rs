// strings2.rs
//
// Faites-moi compiler sans changer la signature de la fonction !
//
// Exécutez `rustlings hint strings2` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

fn main() {
    let word = String::from("green"); // Essayez de ne pas modifier cette ligne :)
    if is_a_color_word(word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
