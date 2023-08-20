// traits2.rs
//
// Votre tâche est d'implémenter le trait `AppendBar` pour un vecteur de chaînes de caractères. Pour
// Pour implémenter ce trait, considérez un instant ce que signifie "ajouter "Bar"
// à un vecteur de chaînes de caractères.
//
// Pas de code de base cette fois-ci, vous pouvez le faire !
//
// Exécutez `rustlings hint traits2` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implémentez le trait `AppendBar` pour un vecteur de chaînes de caractères.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
