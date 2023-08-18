// errors5.rs
//
// Ce programme utilise une version modifiée du code de errors4.
//
// Cet exercice utilise des concepts que nous n'aborderons que plus tard dans le
// cours, comme `Box` et le trait `From`. Il n'est pas important de les comprendre
// en détail pour l'instant, mais vous pouvez lire cela en avance si vous le souhaitez. 
// Pour l'instant, pensez au type `Box<dyn ???>` comme un type "Je veux tout ce qui fait ???", ce qui,
// compte tenu des standards habituels de Rust en matière de sécurité d'exécution, devrait vous sembler
// un peu indulgent !
//
// En bref, ce cas d'utilisation particulier pour les Box est pour quand vous voulez posséder une
// valeur et que vous vous souciez seulement qu'elle soit d'un type qui implémente un trait
// particulier. Pour ce faire, la Box est déclarée comme étant de type Box<dyn Trait> où Trait est
// le trait que le compilateur recherche sur toute valeur utilisée dans ce contexte. Pour cet exercice
// ce contexte est celui des erreurs potentielles qui peuvent être renvoyées dans un
// Result.
//
// Que pouvons-nous utiliser pour décrire ces deux erreurs ? En d'autres termes, existe-t-il un trait
// que les deux erreurs mettent en œuvre ?
//
// Exécutez `rustlings hint errors5` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn ???>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

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

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
