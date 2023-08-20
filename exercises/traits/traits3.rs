// traits3.rs
//
// Votre tâche est d'implémenter le trait Licensed pour les deux structures et de faire en sorte
// qu'elles renvoient les mêmes informations sans écrire deux fois la même fonction.
//
// Réfléchissez à ce que vous pouvez ajouter au trait Licensed.
//
// Exécutez `rustlings hint traits3` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

pub trait Licensed {
    fn licensing_info(&self) -> String;
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Ne pas modifier cette ligne
impl Licensed for OtherSoftware {} // Ne pas modifier cette ligne

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
