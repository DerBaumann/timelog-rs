use crate::entry::{models::Entry, naive_to_utc, repository::EntryRepository};
use askama::Template;
use chrono::{Local, NaiveDate, NaiveDateTime, Utc};
use cli_table::WithTitle;
use crossterm::event::read;
use inquire::{Select, Text};
use std::{collections::BTreeMap, error::Error, fs};

#[derive(Template)]
#[template(path = "export.md")]
struct ExportTemplate {
    entries: BTreeMap<NaiveDate, Vec<Entry>>,
}

pub struct EntryController {
    pub entry_repository: EntryRepository,
}

impl EntryController {
    pub fn record(&self, project: Option<String>) -> Result<(), Box<dyn Error>> {
        let project = match project {
            Some(p) => p,
            None => {
                let mut options = self.entry_repository.fetch_projects()?;
                options.push("Create new!".to_string());

                let p = Select::new("What project are you working on?", options).prompt()?;

                if p == "Create new!" {
                    Text::new("Project name:").prompt()?
                } else {
                    p
                }
            }
        };

        let start_time = Utc::now();
        let local_start = start_time.with_timezone(&Local);

        println!("Started working at: {}", local_start.format("%H:%M"));
        println!("Press enter to stop tracking");

        read()?;

        let end_time = Utc::now();
        let description = Text::new("What did you work on?").prompt()?;

        let entry = Entry::new(project, start_time, end_time, description);
        self.entry_repository.create(entry)?;

        Ok(())
    }

    pub fn export(&self, path: String) -> Result<(), Box<dyn Error>> {
        let entries = self.entry_repository.fetch_all()?;
        let grouped: BTreeMap<NaiveDate, Vec<Entry>> =
            entries.into_iter().fold(BTreeMap::new(), |mut acc, e| {
                let local = e.start_time.with_timezone(&Local);
                acc.entry(local.date_naive()).or_default().push(e);
                acc
            });

        let tmpl = ExportTemplate { entries: grouped };
        let md = tmpl
            .render()
            .expect("Should not error because template is correct");

        fs::write(path, md)?;
        Ok(())
    }

    pub fn list(&self) -> Result<(), Box<dyn Error>> {
        let entries = self.entry_repository.fetch_all()?;
        cli_table::print_stdout(entries.with_title())?;
        Ok(())
    }

    pub fn edit(
        &self,
        id: u32,
        project: Option<String>,
        start_time: Option<NaiveDateTime>,
        end_time: Option<NaiveDateTime>,
        description: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        let start_time = start_time.map(naive_to_utc).transpose()?;
        let end_time = end_time.map(naive_to_utc).transpose()?;

        let mut entry = self.entry_repository.fetch_one(id)?;

        entry.project = project.unwrap_or(entry.project);
        entry.start_time = start_time.unwrap_or(entry.start_time);
        entry.end_time = end_time.unwrap_or(entry.end_time);
        entry.description = description.unwrap_or(entry.description);

        self.entry_repository.update(entry)?;
        println!("Updated Entry {id}");

        Ok(())
    }

    pub fn add(
        &self,
        project: String,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        description: String,
    ) -> Result<(), Box<dyn Error>> {
        let start_time = naive_to_utc(start_time)?;
        let end_time = naive_to_utc(end_time)?;

        let entry = Entry::new(project, start_time, end_time, description);
        self.entry_repository.create(entry)?;

        Ok(())
    }

    pub fn delete(&self, id: u32) -> Result<(), Box<dyn Error>> {
        self.entry_repository.delete(id)?;
        println!("Deleted Entry {id}");
        Ok(())
    }
}
