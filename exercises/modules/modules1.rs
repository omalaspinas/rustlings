// modules1.rs
//
// Exécutez `rustlings hint modules1` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

mod sausage_factory {
    // Ne laissez personne en dehors de ce module voir ceci !
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
