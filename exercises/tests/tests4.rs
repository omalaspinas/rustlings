// tests4.rs
//
// Assurez-vous que nous testons les conditions correctes !
//
// Exécutez `rustlings hint tests4` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Ne modifiez que les fonctions de test elles-mêmes
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // Ce test doit vérifier si le rectangle a la taille que nous passons dans son constructeur
        let rect = Rectangle::new(10, 20);
        assert_eq!(???, 10); // check width
        assert_eq!(???, 20); // check height
    }

    #[test]
    fn negative_width() {
        // Ce test doit permettre de vérifier si le programme panique lorsque nous essayons de créer un rectangle de largeur négative
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    fn negative_height() {
        // Ce test doit permettre de vérifier si le programme panique lorsque nous essayons de créer un rectangle de hauteur négative
        let _rect = Rectangle::new(10, -10);
    }
}
