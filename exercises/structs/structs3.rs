// structs3.rs
//
// Les structures contiennent des données, mais peuvent aussi avoir une logique. 
// Dans cet exercice, nous avons  défini la structure Package et nous voulons 
// tester la logique qui lui est attachée.
// Le code se compile et les tests sont réussis !
//
// Exécutez `rustlings hint structs3` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Package {
        if weight_in_grams < 10 {
            // Ce n'est pas ainsi qu'il faut gérer les erreurs en Rust,
            // mais nous apprendrons à gérer les erreurs plus tard.
            panic!("Il n'est pas possible d'expédier un colis dont le poids est inférieur à 10 grammes.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    fn is_international(&self) -> ??? {
        // Something goes here...
    }

    fn get_fees(&self, cents_per_gram: u32) -> ??? {
        // Something goes here...
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Espagne");
        let recipient_country = String::from("Autriche");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Espagne");
        let recipient_country = String::from("Russie");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Espagne");
        let recipient_country = String::from("Espagne");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
