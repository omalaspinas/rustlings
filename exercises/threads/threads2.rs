// threads2.rs
//
// En nous basant sur le dernier exercice, nous voulons que tous les threads
// terminent leur travail mais cette fois, les threads créés doivent être
// en charge de la mise à jour d'une valeur partagée : JobStatus.jobs_completed
//
// Exécutez `rustlings hint threads2` ou utilisez la sous-commande `hint`
// de `watch` pour obtenir une indication.

// I AM NOT DONE

use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: 0 });
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: Vous devez entreprendre une action avant de mettre à jour une valeur partagée
            status_shared.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Affichez la valeur de JobStatus.jobs_completed. Avez-vous remarqué
        // quelque chose d'intéressant dans la sortie? Devez-vous faire une jointure sur
        // tous les handle?
        println!("jobs completed {}", ???);
    }
}
