// quiz2.rs
//
// Ce quiz couvre les sections suivantes:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Construisons une petite machine sous la forme d'une fonction. Comme
// entrée, nous lui donnerons une liste de chaînes de caractères et de commandes.
// Ces commandes déterminerons quelle action elle appliquera à la chaîne de caractères.
// Les actions seront:
// - Mettre la chaîne de caractères en majuscules.
// - Enlever les espaces de début et de fin de la chaîne.
// - Ajouter "bar" à la fin de la chaîne un nombre de fois déterminé.
// La forme exacte sera:
// - L'entrée sera un Vec d'un tuple: le 1er élément est la chaîne de caractères, le second la commande.
// - La sortie sera un Vec de chaînes de caractères.
//
// Pas d'indices cette fois !
//
// Exécutez `rustlings hint quiz2` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complétez la signature de la fonction !
    pub fn transformer(input: ???) -> ??? {
        // TODO: Complétez la déclaration de sortie !
        let mut output: ??? = vec![];
        for (string, command) in input.iter() {
            // TODO: Complétez le corps de la fonction. Vous pouvez le faire !
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: Que devons-nous importer pour avoir `transformer` dans la portée ?
    use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
