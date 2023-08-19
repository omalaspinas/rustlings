// arc1.rs
//
// Dans cet exercice, on nous donne un Vec de u32 appelé "numbers" avec des valeurs
// allant de 0 à 99 -- [ 0, 1, 2, ..., 98, 99 ] Nous aimerions utiliser ce
// cet ensemble de nombres dans 8 threads différents simultanément. Chaque thread
// va obtenir la somme de chaque huitième valeur, avec un décalage.
//
// Le premier thread (offset 0) fera la somme de 0, 8, 16, ...
// Le deuxième thread (offset 1), fera la somme de 1, 9, 17, ...
// Le troisième thread (offset 2), additionnera 2, 10, 18, ...
// ...
// Le huitième thread (offset 7), additionnera 7, 15, 23, ...
//
// Comme nous utilisons des threads, nos valeurs doivent être thread-safe. C'est pourquoi,
// nous utilisons Arc. Nous devons faire un changement dans chacun des deux TODOs.
//
// Faites compiler ce code en remplissant une valeur pour `shared_numbers` à l'endroit où se trouve 
// le premier commentaire TODO, et en créant un binding initial pour `child_numbers`
// là où se trouve le second commentaire TODO. Essayez de ne pas créer de copies du Vec
// `numbers`!
//
// Exécutez `rustlings hint arc1` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

#![forbid(unused_imports)] // Ne modifiez pas cette ligne (ni la suivante).
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = // TODO
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = // TODO
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
