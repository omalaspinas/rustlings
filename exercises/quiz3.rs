// quiz3.rs
//
// Ce quiz teste :
// - Les génériques
// - Les traits
//
// Une école de magie imaginaire a un nouveau système de génération de bulletin écrit
// en Rust ! Actuellement, le système ne permet de créer que des bulletins où la note
// de l'élève est représentée numériquement (par exemple, 1.0 -> 5.5)
// Cependant, l'école délivre également des notes alphabétiques (A+ -> F-)
// et doit être capable d'imprimer les deux types de bulletins !
//
// Apportez les modifications de code nécessaires à la structure ReportCard et au bloc impl
// pour prendre en charge les bulletins alphabétiques. Remplacez la "grade" dans le second test par
// "A+" pour montrer que vos modifications autorisent les notes alphabétiques.
//
// Exécutez `rustlings hint quiz3` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// J'AI PAS FINI

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Veillez à modifier la note ici après avoir terminé l'exercice.
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
