// primitive_types6.rs
//
// Utiliser un index de tuple pour accéder au second élément de `numbers`. 
// Vous pouvez mettre l'expression pour le deuxième élément à l'endroit où se trouve 
// le ??? pour que le test passe.
//
// Exécutez `rustlings hint primitive_types6` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = ???;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
