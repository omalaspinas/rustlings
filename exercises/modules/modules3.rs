// modules3.rs
//
// Vous pouvez utiliser le mot-clé "use" pour introduire des modules dans votre 
// champ d'application provenant de n'importe où et en particulier de la 
// bibliothèque standard de Rust. Importez SystemTime et UNIX_EPOCH du module std::time. 
// Points de style bonus si vous pouvez le faire en une seule ligne !
//
// Exécutez `rustlings hint modules3` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

// TODO: Complétez cette instruction use
use ???

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
