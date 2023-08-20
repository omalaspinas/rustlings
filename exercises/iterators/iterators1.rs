// iterators1.rs
//
// Lorsqu'on effectue des opérations sur des éléments d'une collection, les itérateurs sont
// essentiels. Ce module vous aide à vous familiariser avec l'utilisation d'un
// itérateur et la manière de parcourir les éléments d'une collection itérable.
//
// Faites-moi compiler en remplissant les `???`
//
// Exécutez `rustlings hint iterators1` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = ???;   // TODO: Step 1

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 4
}
