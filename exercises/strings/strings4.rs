// strings4.rs
//
// Ok, voici un tas de valeurs - certaines sont des `String`, d'autres des `&str`. Votre
// tâche est d'appeler l'une de ces deux fonctions sur chaque valeur en fonction de ce que
// vous pensez que chaque valeur est. En d'autres termes, ajoutez soit `string_slice` soit `string`
// avant les parenthèses sur chaque ligne. Si vous avez raison, le code sera compilé !
//
// Pas d'indices cette fois-ci!
//
// Exécutez `rustlings hint strings4` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    ???("blue");
    ???("red".to_string());
    ???(String::from("hi"));
    ???("rust is fun!".to_owned());
    ???("nice weather".into());
    ???(format!("Interpolation {}", "Station"));
    ???(&String::from("abc")[0..1]);
    ???("  hello there ".trim());
    ???("Happy Monday!".to_string().replace("Mon", "Tues"));
    ???("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
