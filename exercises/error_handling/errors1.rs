// errors1.rs
//
// Cette fonction refuse de générer le texte à imprimer sur un badge si vous lui passez une
// une chaîne vide. Ce serait plus sympa si elle expliquait quel est le problème,
// au lieu de retourner parfois `None`. Heureusement, Rust a une construction similaire
// à `Option` qui peut être utilisée pour exprimer des conditions d'erreur. Utilisons-la!
//
// Exécutez `rustlings hint errors1` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

pub fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // Les noms vides ne sont pas autorisés.
        None
    } else {
        Some(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Ne pas modifier cette ligne
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
