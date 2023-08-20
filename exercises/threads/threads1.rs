// threads1.rs
//
// Ce programme crée plusieurs threads qui s'exécutent chacun pendant au moins 250 ms, et
// chaque thread renvoie le temps qu'il a mis à s'exécuter. Le programme doit
// attendre que tous les threads créés soient terminés et collecter leurs
// valeurs de retour dans un vecteur.
//
// Exécutez `rustlings hint threads1` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: une structure est renvoyée par thread::spawn, pouvez-vous l'utiliser?
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
