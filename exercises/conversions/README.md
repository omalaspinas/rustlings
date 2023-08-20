# Conversions de types

Rust offre une multitude de façons de convertir une valeur d'un type donné en un autre type.

La forme la plus simple de conversion de type est une expression de type cast. Elle est désignée par l'opérateur binaire `as`. Par exemple, `println !("{}", 1 + 1.0);` ne compilerait pas, puisque `1` est un entier alors que `1.0` est un flottant. Cependant, `println !("{}", 1 as f32 + 1.0)` devrait compiler. L'exercice [`using_as`](using_as.rs) essaie de couvrir ce point.

Rust propose également des traits qui facilitent les conversions de type lors de l'implémentation. Ces traits peuvent être trouvés dans le module [`convert`](https://doc.rust-lang.org/std/convert/index.html).
Les traits sont les suivants :

- `From` et `Into` couverts dans [`from_into`](from_into.rs)
- `TryFrom` et `TryInto` couverts dans [`try_from_into`](try_from_into.rs)
- `AsRef` et `AsMut` couverts dans [`as_ref_mut`](as_ref_mut.rs)

De plus, le module `std::str` offre un trait appelé [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) qui aide à convertir les chaînes de caractères en types cibles via la méthode `parse` sur les chaînes de caractères. S'il est correctement implémenté pour un type donné `Person`, alors `let p : Personne = "Mark,20".parse().unwrap()` devrait à la fois compiler et s'exécuter sans paniquer.

Ce sont les principaux moyens ***dans la bibliothèque standard*** de convertir les données dans les types désirés.

## Further information

Ces questions ne sont pas directement abordées dans le livre, mais la bibliothèque standard dispose d'une excellente documentation à ce sujet.

- [Conversions](https://doc.rust-lang.org/std/convert/index.html)
- [Trait `FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)
