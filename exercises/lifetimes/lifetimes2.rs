// lifetimes2.rs
//
// Donc, si le compilateur ne fait que valider les références passées aux
// paramètres annotés et au type de retour, que devons-nous changer ?
//
// Exécutez `rustlings hint lifetimes2` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}
