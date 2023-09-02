use crate::exercise::{Exercise, ExerciseList};
use crate::project::RustAnalyzerProject;
use crate::run::{reset, run};
use crate::verify::verify;
use argh::FromArgs;
use console::Emoji;
use notify::DebouncedEvent;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::ffi::OsStr;
use std::fs;
use std::io::{self, prelude::*};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[macro_use]
mod ui;

mod exercise;
mod project;
mod run;
mod verify;

// In sync with crate version
const VERSION: &str = "5.5.1";

#[derive(FromArgs, PartialEq, Debug)]
/// Rustlings is a collection of small exercises to get you used to writing and reading Rust code
struct Args {
    /// show outputs from the test exercises
    #[argh(switch)]
    nocapture: bool,
    /// show the executable version
    #[argh(switch, short = 'v')]
    version: bool,
    #[argh(subcommand)]
    nested: Option<Subcommands>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    Verify(VerifyArgs),
    Watch(WatchArgs),
    Run(RunArgs),
    Reset(ResetArgs),
    Hint(HintArgs),
    List(ListArgs),
    Lsp(LspArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "verify")]
/// Verifies all exercises according to the recommended order
struct VerifyArgs {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "watch")]
/// Reruns `verify` when files were edited
struct WatchArgs {
    /// show hints on success
    #[argh(switch)]
    success_hints: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
/// Runs/Tests a single exercise
struct RunArgs {
    #[argh(positional)]
    /// the name of the exercise
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "reset")]
/// Resets a single exercise using "git stash -- <filename>"
struct ResetArgs {
    #[argh(positional)]
    /// the name of the exercise
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "hint")]
/// Returns a hint for the given exercise
struct HintArgs {
    #[argh(positional)]
    /// the name of the exercise
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "lsp")]
/// Enable rust-analyzer for exercises
struct LspArgs {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "list")]
/// Lists the exercises available in Rustlings
struct ListArgs {
    #[argh(switch, short = 'p')]
    /// show only the paths of the exercises
    paths: bool,
    #[argh(switch, short = 'n')]
    /// show only the names of the exercises
    names: bool,
    #[argh(option, short = 'f')]
    /// provide a string to match exercise namescommande
    /// comma separated patterns are acceptable
    filter: Option<String>,
    #[argh(switch, short = 'u')]
    /// display only exercises not yet solved
    unsolved: bool,
    #[argh(switch, short = 's')]
    /// display only exercises that have been solved
    solved: bool,
}

fn main() {
    let args: Args = argh::from_env();

    if args.version {
        println!("v{VERSION}");
        std::process::exit(0);
    }

    if args.nested.is_none() {
        println!("\n{WELCOME}\n");
    }

    if !Path::new("info.toml").exists() {
        println!(
            "{} doit être exécuté à partir du répertoire rustlings",
            std::env::current_exe().unwrap().to_str().unwrap()
        );
        println!("Try `cd rustlings/`!");
        std::process::exit(1);
    }

    if !rustc_exists() {
        println!("Nous ne trouvons pas `rustc`.");
        println!("Essayez de lancer `rustc --version` pour diagnostiquer votre problème.");
        println!("Pour savoir comment installer Rust, consultez le README.");
        std::process::exit(1);
    }

    let toml_str = &fs::read_to_string("info.toml").unwrap();
    let exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;
    let verbose = args.nocapture;

    let command = args.nested.unwrap_or_else(|| {
        println!("{DEFAULT_OUT}\n");
        std::process::exit(0);
    });
    match command {
        Subcommands::List(subargs) => {
            if !subargs.paths && !subargs.names {
                println!("{:<17}\t{:<46}\t{:<7}", "Name", "Path", "Status");
            }
            let mut exercises_done: u16 = 0;
            let filters = subargs.filter.clone().unwrap_or_default().to_lowercase();
            exercises.iter().for_each(|e| {
                let fname = format!("{}", e.path.display());
                let filter_cond = filters
                    .split(',')
                    .filter(|f| !f.trim().is_empty())
                    .any(|f| e.name.contains(f) || fname.contains(f));
                let status = if e.looks_done() {
                    exercises_done += 1;
                    "Terminé"
                } else {
                    "En attente"
                };
                let solve_cond = {
                    (e.looks_done() && subargs.solved)
                        || (!e.looks_done() && subargs.unsolved)
                        || (!subargs.solved && !subargs.unsolved)
                };
                if solve_cond && (filter_cond || subargs.filter.is_none()) {
                    let line = if subargs.paths {
                        format!("{fname}\n")
                    } else if subargs.names {
                        format!("{}\n", e.name)
                    } else {
                        format!("{:<17}\t{fname:<46}\t{status:<7}\n", e.name)
                    };
                    // Somehow using println! leads to the binary panicking
                    // when its output is piped.
                    // So, we're handling a Broken Pipe error and exiting with 0 anyway
                    let stdout = std::io::stdout();
                    {
                        let mut handle = stdout.lock();
                        handle.write_all(line.as_bytes()).unwrap_or_else(|e| {
                            match e.kind() {
                                std::io::ErrorKind::BrokenPipe => std::process::exit(0),
                                _ => std::process::exit(1),
                            };
                        });
                    }
                }
            });
            let percentage_progress = exercises_done as f32 / exercises.len() as f32 * 100.0;
            println!(
                "Progression : Vous avez effectué {} / {} exercices ({:.1} %).",
                exercises_done,
                exercises.len(),
                percentage_progress
            );
            std::process::exit(0);
        }

        Subcommands::Run(subargs) => {
            let exercise = find_exercise(&subargs.name, &exercises);

            run(exercise, verbose).unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Reset(subargs) => {
            let exercise = find_exercise(&subargs.name, &exercises);

            reset(exercise).unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Hint(subargs) => {
            let exercise = find_exercise(&subargs.name, &exercises);

            println!("{}", exercise.hint);
        }

        Subcommands::Verify(_subargs) => {
            verify(&exercises, (0, exercises.len()), verbose, false)
                .unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Lsp(_subargs) => {
            let mut project = RustAnalyzerProject::new();
            project.get_sysroot_src().expect(
                "Impossible de trouver le path de la toolchain, avez-vous installé `rustc` ?",
            );
            project
                .exercises_to_json()
                .expect("Impossible d'analyser les fichiers d'exercices de rustlings");

            if project.crates.is_empty() {
                println!("Impossible de trouver des exercices, vérifiez que vous êtes bien dans le dossier `rustlings`");
            } else if project.write_to_disk().is_err() {
                println!(
                    "Échec de l'écriture de rust-project.json sur le disque pour rust-analyzer"
                );
            } else {
                println!("Génération réussie de rust-project.json");
                println!("rust-analyzer va maintenant analyser les exercices, redémarrez votre serveur de langue ou votre éditeur")
            }
        }

        Subcommands::Watch(_subargs) => match watch(&exercises, verbose, _subargs.success_hints) {
            Err(e) => {
                println!(
                    "Erreur : Impossible de suivre votre progression. Le message d'erreur était {:?}.",
                    e
                );
                println!("Il est probable que vous n'ayez plus d'espace disque ou que la limite de votre 'inotify' ait été atteinte.");
                std::process::exit(1);
            }
            Ok(WatchStatus::Finished) => {
                println!(
                    "{emoji} Tous les exercices sont terminés ! {emoji}",
                    emoji = Emoji("🎉", "★")
                );
                println!("\n{FENISH_LINE}\n");
            }
            Ok(WatchStatus::Unfinished) => {
                println!("Nous espérons que vous prenez plaisir à découvrir le Rust !");
                println!("Si vous voulez continuer à travailler sur les exercices plus tard, vous pouvez simplement exécuter `rustlings watch` à nouveau");
            }
        },
    }
}

fn spawn_watch_shell(
    failed_exercise_hint: &Arc<Mutex<Option<String>>>,
    should_quit: Arc<AtomicBool>,
) {
    let failed_exercise_hint = Arc::clone(failed_exercise_hint);
    println!("Bienvenue dans le mode 'watch' ! Vous pouvez taper 'help' pour obtenir une vue d'ensemble des commandes que vous pouvez utiliser.");
    thread::spawn(move || loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input == "hint" {
                    if let Some(hint) = &*failed_exercise_hint.lock().unwrap() {
                        println!("{hint}");
                    }
                } else if input == "clear" {
                    println!("\x1B[2J\x1B[1;1H");
                } else if input.eq("quit") {
                    should_quit.store(true, Ordering::SeqCst);
                    println!("Au revoir !");
                } else if input.eq("help") {
                    println!("Commandes disponibles en mode 'watch':");
                    println!("  hint   - affiche l'indice de l'exercice en cours");
                    println!("  clear  - efface l'écran");
                    println!("  quit   - quitte le mode veille");
                    println!("  !<cmd> - exécute une commande, comme `!rustc --explain E0381`");
                    println!("  help   - affiche ce message d'aide");
                    println!();
                    println!("Le mode veille réévalue automatiquement l'exercice en cours");
                    println!("lorsque vous modifiez le contenu d'un fichier.")
                } else if let Some(cmd) = input.strip_prefix('!') {
                    let parts: Vec<&str> = cmd.split_whitespace().collect();
                    if parts.is_empty() {
                        println!("aucune commande n'est fournie");
                    } else if let Err(e) = Command::new(parts[0]).args(&parts[1..]).status() {
                        println!("échec de l'exécution de la commande `{}`: {}", cmd, e);
                    }
                } else {
                    println!("commande inconnue: {input}");
                }
            }
            Err(error) => println!("erreur de lecture de la commande : {error}"),
        }
    });
}

fn find_exercise<'a>(name: &str, exercises: &'a [Exercise]) -> &'a Exercise {
    if name.eq("next") {
        exercises
            .iter()
            .find(|e| !e.looks_done())
            .unwrap_or_else(|| {
                println!("🎉 Félicitations ! Vous avez fait tous les exercices !");
                println!("🔚 Il n'y a plus d'exercices à faire ensuite !");
                std::process::exit(1)
            })
    } else {
        exercises
            .iter()
            .find(|e| e.name == name)
            .unwrap_or_else(|| {
                println!("Aucun exercice n'a été trouvé pour '{name}'!");
                std::process::exit(1)
            })
    }
}

enum WatchStatus {
    Finished,
    Unfinished,
}

fn watch(
    exercises: &[Exercise],
    verbose: bool,
    success_hints: bool,
) -> notify::Result<WatchStatus> {
    /* Clears the terminal with an ANSI escape code.
    Works in UNIX and newer Windows terminals. */
    fn clear_screen() {
        println!("\x1Bc");
    }

    let (tx, rx) = channel();
    let should_quit = Arc::new(AtomicBool::new(false));

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;
    watcher.watch(Path::new("./exercises"), RecursiveMode::Recursive)?;

    clear_screen();

    let to_owned_hint = |t: &Exercise| t.hint.to_owned();
    let failed_exercise_hint = match verify(
        exercises.iter(),
        (0, exercises.len()),
        verbose,
        success_hints,
    ) {
        Ok(_) => return Ok(WatchStatus::Finished),
        Err(exercise) => Arc::new(Mutex::new(Some(to_owned_hint(exercise)))),
    };
    spawn_watch_shell(&failed_exercise_hint, Arc::clone(&should_quit));
    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => match event {
                DebouncedEvent::Create(b) | DebouncedEvent::Chmod(b) | DebouncedEvent::Write(b) => {
                    if b.extension() == Some(OsStr::new("rs")) && b.exists() {
                        let filepath = b.as_path().canonicalize().unwrap();
                        let pending_exercises = exercises
                            .iter()
                            .find(|e| filepath.ends_with(&e.path))
                            .into_iter()
                            .chain(
                                exercises
                                    .iter()
                                    .filter(|e| !e.looks_done() && !filepath.ends_with(&e.path)),
                            );
                        let num_done = exercises.iter().filter(|e| e.looks_done()).count();
                        clear_screen();
                        match verify(
                            pending_exercises,
                            (num_done, exercises.len()),
                            verbose,
                            success_hints,
                        ) {
                            Ok(_) => return Ok(WatchStatus::Finished),
                            Err(exercise) => {
                                let mut failed_exercise_hint = failed_exercise_hint.lock().unwrap();
                                *failed_exercise_hint = Some(to_owned_hint(exercise));
                            }
                        }
                    }
                }
                _ => {}
            },
            Err(RecvTimeoutError::Timeout) => {
                // the timeout expired, just check the `should_quit` variable below then loop again
            }
            Err(e) => println!("erreur de 'watch': {e:?}"),
        }
        // Check if we need to exit
        if should_quit.load(Ordering::SeqCst) {
            return Ok(WatchStatus::Unfinished);
        }
    }
}

fn rustc_exists() -> bool {
    Command::new("rustc")
        .args(["--version"])
        .stdout(Stdio::null())
        .spawn()
        .and_then(|mut child| child.wait())
        .map(|status| status.success())
        .unwrap_or(false)
}

const DEFAULT_OUT: &str = r#"Merci d'avoir installé Rustlings !

C'est votre première fois ? Ne vous inquiétez pas, Rustlings a été conçu pour les débutants ! Nous allons
allons vous apprendre beaucoup de choses sur Rust, 
mais avant de commencer, voici quelques remarques sur le fonctionnement de Rustlings :

1. Le concept central de Rustlings est que vous résolvez des exercices. Ces
   exercices contiennent généralement une erreur de syntaxe, ce qui les fait
   de les faire échouer à la compilation ou au test. Parfois, il y a une erreur de logique
   au lieu d'une erreur de syntaxe. Quelle que soit l'erreur, c'est à vous de la trouver et de la corriger !
   Vous saurez que vous l'avez corrigée car l'exercice compilera et
   Rustlings pourra passer à l'exercice suivant.
2. Si vous lancez Rustlings en mode "watch" (ce que nous recommandons), il commencera automatiquement
   automatiquement par le premier exercice. Ne vous laissez pas déconcerter par un message 
   d'erreur qui apparaît dès que vous lancez Rustlings ! Il s'agit d'une partie de l'exercice que vous devez résoudre
   alors ouvrez le fichier de l'exercice dans un éditeur et commencez votre travail de détective !
   travail de détective !
3. Si vous êtes bloqués sur un exercice, il y a un indice que vous pouvez consulter en tapant
   'hint' (en mode veille), ou en exécutant `rustlings hint nom_de_l_exercice`.
4. Si un exercice n'a pas de sens pour vous, n'hésitez pas à ouvrir une issue sur GitHub !
   (https://github.com/rust-lang/rustlings/issues/new). Nous examinons chaque issue,
   et parfois, d'autres apprenants le font aussi, donc vous pouvez vous aider mutuellement !
5. Si vous voulez utiliser `rust-analyzer` avec les exercices, qui fournit des fonctionnalités comme
   l'autocomplétion, lancez la commande `rustlings lsp`.

Vous avez compris tout cela ? C'est parfait ! Pour commencer, lancez la commande `rustlings watch` afin d'obtenir le premier exercice
exercice. Assurez-vous que votre éditeur est ouvert !"#;

const FENISH_LINE: &str = r"+----------------------------------------------------+
|        Vous avez atteint la ligne d'arrivée!       |
+--------------------------  ------------------------+
                          \\/
     ▒▒          ▒▒▒▒▒▒▒▒      ▒▒▒▒▒▒▒▒          ▒▒
   ▒▒▒▒  ▒▒    ▒▒        ▒▒  ▒▒        ▒▒    ▒▒  ▒▒▒▒
   ▒▒▒▒  ▒▒  ▒▒            ▒▒            ▒▒  ▒▒  ▒▒▒▒
 ░░▒▒▒▒░░▒▒  ▒▒            ▒▒            ▒▒  ▒▒░░▒▒▒▒
   ▓▓▓▓▓▓▓▓  ▓▓      ▓▓██  ▓▓  ▓▓██      ▓▓  ▓▓▓▓▓▓▓▓
     ▒▒▒▒    ▒▒      ████  ▒▒  ████      ▒▒░░  ▒▒▒▒
       ▒▒  ▒▒▒▒▒▒        ▒▒▒▒▒▒        ▒▒▒▒▒▒  ▒▒
         ▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▒▒▓▓▒▒▒▒▒▒▒▒
           ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
             ▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒██▒▒▒▒▒▒▒▒▒▒
           ▒▒  ▒▒▒▒▒▒▒▒▒▒██████▒▒▒▒▒▒▒▒▒▒  ▒▒
         ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒
       ▒▒    ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒    ▒▒
       ▒▒  ▒▒    ▒▒                  ▒▒    ▒▒  ▒▒
           ▒▒  ▒▒                      ▒▒  ▒▒

Nous espérons que vous avez apprécié l'apprentissage des différents aspects de Rust !
Si vous avez remarqué des problèmes, n'hésitez pas à les signaler dans notre repo.
Vous pouvez également contribuer à nos exercices pour aider la communauté !

Avant de signaler un problème ou de contribuer, veuillez lire nos directives :
https://github.com/rust-lang/rustlings/blob/main/CONTRIBUTING.md";

const WELCOME: &str = r"       bienvenue à...
                 _   _ _
  _ __ _   _ ___| |_| (_)_ __   __ _ ___
 | '__| | | / __| __| | | '_ \ / _` / __|
 | |  | |_| \__ \ |_| | | | | | (_| \__ \
 |_|   \__,_|___/\__|_|_|_| |_|\__, |___/
                               |___/";
