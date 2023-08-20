// functions4.rs
//
// Ce magasin fait des soldes où si le prix est un nombre pair, vous obtenez 
// un rabais de 10 Rustbucks, mais si c'est un nombre impair, c'est 3 
// Rustbucks de réduction. 
// (Ne vous inquiétez pas du corps des fonctions, seules les signatures nous 
// intéressent pour l'instant. C'est un bon moyen d'anticiper les exercices à 
// venir).
//
// Exécutez `rustlings hint functions4` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
