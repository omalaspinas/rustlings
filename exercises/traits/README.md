# Traits

Un trait est un ensemble de méthodes.

Les types peuvent implémenter des traits.
Pour ce faire, les méthodes qui composent les traits sont définies pour le type de données. 
Par exemple, le type de données `String` implémente le trait `From<&str>`. 
Cela permet à un utilisateur d'écrire `String::from("hello")`.

De cette manière, les traits sont quelque peu similaires aux interfaces Java et aux classes abstraites C++.

Parmi les traits Rust les plus courants, on peut citer

- `Clone` (la méthode `clone`)
- `Display` (qui permet l'affichage formaté via `{}`)
- `Debug` (qui permet l'affichage formaté via `{:?}`)

Parce que les traits indiquent un comportement partagé entre les types de données, ils sont utiles lors de l'écriture de génériques.

## Plus d'informations

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
