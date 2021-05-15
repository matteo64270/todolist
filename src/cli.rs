use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Ecrire les tache dans le journal de tache
    Add {
        /// Description de la tache
        #[structopt()]
        text: String,
    },
    /// Supprimer une tache en fonction de sa position
    Done {
        #[structopt()]
        position: usize,
    },
    /// Lister les toutes les taches
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
