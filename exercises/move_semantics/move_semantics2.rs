// move_semantics2.rs
//
// Résultats attendus:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Exécutez `rustlings hint move_semantics2` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!(
        "{} has length {}, with contents: `{:?}`",
        "vec0",
        vec0.len(),
        vec0
    );

    vec1.push(88);

    println!(
        "{} has length {}, with contents `{:?}`",
        "vec1",
        vec1.len(),
        vec1
    );
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
