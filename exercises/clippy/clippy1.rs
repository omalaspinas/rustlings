// clippy1.rs
//
// L'outil Clippy est une collection de lints pour analyser votre code afin que vous puissiez
// détecter les erreurs communes et améliorer votre code Rust
//
// Pour ces exercices, le code ne se compilera pas quand il y a des avertissements de Clippy
// Vérifiez les suggestions de Clippy à partir pour résoudre l'exercice.
//
// Exécutez `rustlings hint clippy1` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

use std::f32;

fn main() {
    let pi = 3.14f32;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
