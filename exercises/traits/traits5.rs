// traits5.rs
//
// Votre tâche consiste à remplacer les sections '??' pour que le code compile.
//
// Ne changez pas d'autres lignes que celles qui sont marquées.
//
// Exécutez `rustlings hint traits5` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// VOUS NE POUVEZ MODIFIER QUE LA LIGNE SUIVANTE
fn some_func(item: ??) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}
