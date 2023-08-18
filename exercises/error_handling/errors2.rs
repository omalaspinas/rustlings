// errors2.rs
//
// Imaginons que nous écrivions un jeu dans lequel vous pouvez acheter des objets avec des jetons. Tous les objets coûtent
// 5 tokens, et chaque fois que vous achetez des objets, il y a des frais de traitement de 1
// token. Un joueur du jeu tape le nombre d'objets qu'il veut acheter.
// La fonction `total_cost` calculera le coût total de tokens. Puisque
// le joueur a saisi la quantité d'objets souhaitée, nous l'obtenons sous
// la forme d'une chaîne de caractères et il
// peut avoir tapé n'importe quoi, pas seulement des nombres !
//
// Pour l'instant, cette fonction ne gère pas du tout le cas d'erreurs (et ne gère
// pas le cas de succès correctement non plus). Ce que nous voulons faire est : si nous appelons
// la fonction `total_cost` sur une chaîne qui n'est pas un nombre, cette fonction
// retournera une `ParseIntError`, et dans ce cas, nous voulons immédiatement
// retourner cette erreur de notre fonction et ne pas essayer de multiplier et d'ajouter.
//
// Il y a au moins deux façons d'implémenter ceci qui sont toutes les deux correctes,
// mais l'une d'entre elles est beaucoup plus courte !
//
// Exécutez `rustlings hint errors2` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
