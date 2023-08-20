// errors6.rs
//
// L'utilisation de types d'erreurs fourre-tout comme `Box<dyn error::Error>` n'est pas recommandée
// pour le code de la bibliothèque, où les appelants pourraient vouloir prendre des décisions basées
// sur le contenu de l'erreur, au lieu de l'imprimer ou de la propager plus loin. Ici, nous
// définissons un type d'erreur personnalisé pour permettre aux appelants de
// décider de ce qu'il faut faire lorsque notre fonction renvoie une erreur.
//
// Exécutez `rustlings hint errors6` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

use std::num::ParseIntError;

// C'est un type d'erreur personnalisé que nous utiliserons dans `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // TODO: ajoutez une autre fonction de conversion d'erreur ici.
    // fn from_parseint...
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // TODO: changez ceci pour retourner une erreur appropriée au lieu de paniquer
    // lorsque `parse()` renvoie une erreur.
    let x: i64 = s.parse().unwrap();
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}

// Ne modifiez rien en dessous de cette ligne.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // Nous ne pouvons pas construire une erreur de type ParseIntError, donc nous devons pattern matcher.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
