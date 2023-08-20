// macros3.rs
//
// Faites-moi compiler, sans sortir la macro du module!
//
// Exécutez `rustlings hint macros3` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
