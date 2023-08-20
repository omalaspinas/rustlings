// iterators3.rs
//
// Il s'agit d'un exercice plus long que la plupart des autres! Vous pouvez le faire! Voici
// votre mission, si vous l'acceptez:
// 1. complétez la fonction divide pour réussir les quatre premiers tests.
// 2. complétez les fonctions result_with_list et list_of_results pour réussir les autres tests.
//
// Exécutez `rustlings hint iterators3` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// Calculez `a` divisé par `b` si `a` est divisible par `b`.
// Sinon, retournez une erreur appropriée.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    todo!();
}

// Complétez la fonction et renvoyez une valeur du bon type pour que le test
// passe.
// Sortie souhaitée : Ok([1, 11, 1426, 3])
fn result_with_list() -> () {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
}

// Complétez la fonction et renvoyez une valeur du bon type pour que le test
// passe.
// Sortie souhaitée : [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> () {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
