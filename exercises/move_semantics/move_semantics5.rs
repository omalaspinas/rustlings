// move_semantics5.rs
//
// Faites-moi compiler seulement en réordonnant les lignes dans `main()`,
// mais sans ajouter, modifier ou supprimer aucune d'entre elles.
//
// Exécutez `rustlings hint move_semantics5` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

fn main() {
    let mut x = 100;
    let y = &mut x;
    let z = &mut x;
    *y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
}
