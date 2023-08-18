# Lifetimes

Les durées de vie indiquent au compilateur comment vérifier si les références vivent 
suffisamment longtemps pour être valables dans une situation donnée. Par exemple, les durées de vie disent
"Assurez-vous que le paramètre 'a' vit aussi longtemps que le paramètre 'b' pour que la valeur de retour
soit valide".

Elles ne sont nécessaires que pour les emprunts, c'est-à-dire les références,
puisque les paramètres copiés ou les move sont possédées dans leur portée et ne peuvent pas être
être référencés à l'extérieur. Les durées de vie signifient que le code d'appel des fonctions, 
par exemple, peut être vérifié pour s'assurer que leurs arguments sont valides. Les durées de vie sont
restrictives pour leurs appelants.

Si vous souhaitez en savoir plus sur les annotations de durée de vie, le projet
projet [lifetimekata](https://tfpk.github.io/lifetimekata/)
a un style d'exercices similaire à celui de Rustlings, 
mais il s'agit d'apprendre à écrire des annotations de durée de vie.

## Further information

- [Lifetimes (dans Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Validation des références avec les durées de vie](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
