// move_semantics4.rs
//
// Refasctorisez ce code pour qu'au lieu de passer `vec0` dans la fonction
// `fill_vec` le vecteur est créé dans la fonction elle-même et renvoyé à la
// fonction principale.
//
// Exécutez `rustlings hint move_semantics4` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
