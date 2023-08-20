// quiz1.rs
//
// Il s'agit d'un quiz pour les sections suivantes:
// - Variables
// - Fonctions
// - If
//
// Marie achète des pommes. Le prix d'une pomme est calculé comme suit :
// - Une pomme coûte 2 rustbucks.
// - Si Marie achète plus de 40 pommes, chaque pomme ne coûte que 1 rustbuck !
// Écrivez une fonction qui calcule le prix d'une commande de pommes en fonction de
// la quantité achetée.
//
// Pas d'indice cette fois ;)

// J'AI PAS FINI

// Placez votre fonction ici!
// fn calculate_price_of_apples {

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
