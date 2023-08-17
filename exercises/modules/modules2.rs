// modules2.rs
//
// Vous pouvez intégrer les chemins d'accès aux modules dans les portées et 
// leur donner de nouveaux noms à l'aide des mots-clés "use" et "as".
// Corrigez ces déclarations "use" pour que le code compile.
//
// Exécutez `rustlings hint modules2` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

mod delicious_snacks {
    // TODO: Corriger ces instructions use
    use self::fruits::PEAR as ???
    use self::veggies::CUCUMBER as ???

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
