// traits1.rs
//
// Il est temps d'implémenter quelques traits ! Votre tâche est d'implémenter le trait
// `AppendBar` pour le type `String`. Le trait AppendBar n'a qu'une seule fonction,
// qui ajoute "Bar" à tout objet implémentant ce trait.
//
// Exécutez `rustlings hint traits1` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implémentez `AppendBar` pour le type `String`.
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
