// hashmaps3.rs
//
// Une liste de scores (un par ligne) d'un match de football est donnée. Chaque ligne est de la
// de la forme : "<nom_de_l'équipe_1>,<nom_de_l'équipe_2>,<objectifs_de_l'équipe_1>,<objectifs_de_l'équipe_2>"
// Exemple : Angleterre,France,4,2 (l'Angleterre a marqué 4 buts, la France 2).
//
// Vous devez construire un tableau de scores contenant le nom de l'équipe, les
// buts marqués par l'équipe et les buts encaissés par l'équipe. Une approche pour construire le tableau de
// est d'utiliser une Hashmap. La solution est partiellement écrite pour utiliser une
// Hashmap, complétez-la pour qu'elle passe le test.
//
// Faites-moi passer les tests !
//
// Exécutez `rustlings hint hashmaps3` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

use std::collections::HashMap;

// Une structure pour stocker les détails des buts d'une équipe.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // Le nom de l'équipe est la clé et sa structure associée est la valeur.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO : Remplir le tableau des scores avec les détails extraits de la
        // ligne en cours. Gardez à l'esprit que les buts marqués par l'équipe_1
        // seront le nombre de buts encaissés par l'équipe_2, et de la même manière
        // les buts marqués par l'équipe_2 seront le nombre de buts encaissés par
        // l'équipe_1.
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Espagne,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "Espagne", "France", "Germany", "Italy", "Poland"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Espagne").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
