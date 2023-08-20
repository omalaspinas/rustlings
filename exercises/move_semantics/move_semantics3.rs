// move_semantics3.rs
//
// Faites-moi compiler sans ajouter de nouvelles lignes - juste en changeant
// les lignes existantes ! (les lignes avec plusieurs points-virgules
// ne sont pas nécessaires).
//
// Exécutez `rustlings hint move_semantics3` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
