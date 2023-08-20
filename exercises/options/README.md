# Options

Le type Option représente une valeur optionnelle : chaque Option est soit Some et contient une valeur, soit None et n'en contient pas.
Les types Option sont très courants dans le code Rust, car ils ont de nombreuses utilisations :

- Valeurs initiales
- Valeurs de retour pour les fonctions qui ne sont pas définies sur l'ensemble de leur plage d'entrée (fonctions partielles)
- Valeur de retour pour signaler des erreurs simples, où None est renvoyé en cas d'erreur
- Champs de structure optionnels
- Champs de structure qui peuvent être prêtés ou "pris"
- Arguments de fonction facultatifs
- Pointeurs annulables
- Échanger des éléments pour sortir de situations difficiles

## Plus d'informations

- [L'enum Option](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-enum-definitions)
- [Documentation du module option](https://doc.rust-lang.org/std/option/)
- [Documentation de l'enum Option](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
