use crate::{core::repository::Repository, entry::models::Entry};
use askama::Template;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone, Timelike};
use cli_table::{WithTitle, print_stdout};
use crossterm::event::read;
use inquire::Text;
use std::{collections::BTreeMap, error::Error, fs};
use thiserror::Error;

#[derive(Template)]
#[template(path = "export.md")]
struct ExportTemplate {
    entries: BTreeMap<NaiveDate, Vec<Entry>>,
}

#[derive(Debug, Error)]
pub enum TimeError {
    #[error("Invalid or ambiguous time")]
    Invalid,
}

pub struct EntryController {
    pub entry_repository: Box<dyn Repository<Entry>>,
}

impl EntryController {
    fn to_local(t: NaiveDateTime) -> Result<DateTime<Local>, TimeError> {
        Local
            .from_local_datetime(&t)
            .single()
            .ok_or(TimeError::Invalid)
    }

    pub fn record(&self, project: Option<String>) -> Result<(), Box<dyn Error>> {
        let project = match project {
            Some(p) => p,
            // TODO: Change this to select from all existing projects or add new
            None => Text::new("What project are you working on?").prompt()?,
        };

        let start_time = Local::now();
        let start_time_str = format!("{:02}:{:02}", start_time.hour(), start_time.minute());

        println!("Started working at: {}", start_time_str);
        println!("Press enter to stop tracking");

        read()?;

        let end_time = Local::now();
        let description = Text::new("What did you work on?").prompt()?;

        let entry = Entry::new(project, start_time, end_time, description);
        self.entry_repository.create(entry)?;

        Ok(())
    }

    pub fn export(&self, path: String) -> Result<(), Box<dyn Error>> {
        let entries = self.entry_repository.fetch_all()?;
        let grouped: BTreeMap<NaiveDate, Vec<Entry>> =
            entries.into_iter().fold(BTreeMap::new(), |mut acc, e| {
                acc.entry(e.start_time.date_naive()).or_default().push(e);
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
        print_stdout(entries.with_title())?;
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
        let start_time = start_time.and_then(|t| Self::to_local(t).ok());
        let end_time = end_time.and_then(|t| Self::to_local(t).ok());

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
        let start_time = Self::to_local(start_time)?;
        let end_time = Self::to_local(end_time)?;

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
