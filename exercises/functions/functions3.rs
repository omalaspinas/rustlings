// functions3.rs
//
// Exécutez `rustlings hint functions3` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

fn main() {
    call_me();
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
