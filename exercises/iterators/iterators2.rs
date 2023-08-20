// iterators2.rs
//
// Dans cet exercice, vous découvrirez certains des avantages que les itérateurs
// peuvent offrir. Suivez les étapes pour réaliser l'exercice.
//
// Exécutez `rustlings hint iterators2` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

// Étape 1.
// Complétez la fonction `capitalize_first`.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => ???,
    }
}

// Étape 2.
// Appliquez la fonction `capitalize_first` à un slice de chaîne de caractères.
// Retourne un vecteur de chaînes de caractères.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    vec![]
}

// Étape 3.
// Appliquez à nouveau la fonction `capitalize_first` à un slice de chaîne de caractères.
// Retourne une seule chaîne de caractères.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
