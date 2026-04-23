use chrono::NaiveDateTime;
use clap::{Parser, Subcommand};
use std::{env, error::Error, path::PathBuf};
use timelog::entry::{controller::EntryController, repository::EntryRepository};

// TODO: Change the name to not conflict with existing crates
// TODO: Maybe more features to make this more useful
// TODO: Add documentation comments to project: https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Clone, Subcommand)]
enum Commands {
    Record {
        project: Option<String>,
    },
    Export {
        path: String,
    },
    List,
    Edit {
        id: u32,

        #[arg(short, long)]
        project: Option<String>,
        #[arg(short, long)]
        start_time: Option<NaiveDateTime>,
        #[arg(short, long)]
        end_time: Option<NaiveDateTime>,
        #[arg(short, long)]
        description: Option<String>,
    },
    Add {
        #[arg(short, long)]
        project: String,
        #[arg(short, long)]
        start_time: NaiveDateTime,
        #[arg(short, long)]
        end_time: NaiveDateTime,
        #[arg(short, long)]
        description: String,
    },
    Delete {
        id: u32,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let entry_controller = EntryController {
        entry_repository: EntryRepository {
            file_path: env::var_os("TIMELOG_STOREPATH")
                .map(PathBuf::from)
                .or_else(|| dirs::config_local_dir().map(|p| p.join("store.json")))
                .ok_or("TIMELOG_STOREPATH and user config dirs are both not defined!")?,
        },
    };

    match cli.command {
        Some(Commands::Record { project }) => entry_controller.record(project),
        Some(Commands::Export { path }) => entry_controller.export(path),
        Some(Commands::Edit {
            id,
            project,
            start_time,
            end_time,
            description,
        }) => entry_controller.edit(id, project, start_time, end_time, description),
        Some(Commands::Add {
            project,
            start_time,
            end_time,
            description,
        }) => entry_controller.add(project, start_time, end_time, description),
        Some(Commands::Delete { id }) => entry_controller.delete(id),
        None | Some(Commands::List) => entry_controller.list(),
    }
}
