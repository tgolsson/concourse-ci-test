/*!

*/

use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::time::{Duration, SystemTime};

use crate::api::Event;

pub struct DbEvent {
    pub message: String,
    pub app: String,
    pub tags: Vec<i64>,
    pub epoch_seconds: i64,
}

pub struct DbTag {
    pub text: String,
}

impl From<String> for DbTag {
    fn from(s: String) -> Self {
        Self { text: s }
    }
}

pub struct DatabaseApi {
    connection: Connection,
}

impl DatabaseApi {
    pub fn new_temporary() -> Result<Self> {
        let connection = Connection::open_in_memory()?;

        Ok(Self { connection })
    }

    pub fn init_database(&self) -> Result<()> {
        self.connection
            .execute(
                "CREATE TABLE status (
                  id              INTEGER PRIMARY KEY,
                  message         TEXT NOT NULL,
                  app             TEXT,
                  timestamp       INTEGER
                  )",
                params![],
            )
            .context("creating status table")?;

        self.connection
            .execute(
                "CREATE TABLE tags (
                  id           INTEGER PRIMARY KEY,
                  text         TEXT NOT NULL UNIQUE
                  )",
                params![],
            )
            .context("creating tags table")?;

        self.connection
            .execute(
                "CREATE TABLE tags_relation (
                  status_id    INTEGER,
                  tag_id       INTEGER,
                  FOREIGN KEY (status_id) REFERENCES status(id),
                  FOREIGN KEY (tag_id) REFERENCES tags(id),
                  PRIMARY KEY (status_id, tag_id)
                  )",
                params![],
            )
            .context("creating tags relation")?;
        Ok(())
    }

    pub fn add_fake_data(&self) -> Result<()> {
        let fake_tags: Vec<DbTag> = vec![
            "build".to_owned().into(),
            "js".to_owned().into(),
            "go".to_owned().into(),
            "helm".to_owned().into(),
            "deploy".to_owned().into(),
            "automation".to_owned().into(),
        ];

        let seconds_since_epoch = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs() as i64,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };

        let fake_events = vec![
            DbEvent {
                message: "Starting a build".to_owned(),
                app: "concourse".to_owned(),
                tags: vec![0, 1, 3],
                epoch_seconds: seconds_since_epoch,
            },
            DbEvent {
                message: "Deploying helm".to_owned(),
                app: "git".to_owned(),
                tags: vec![3, 4, 2],
                epoch_seconds: seconds_since_epoch + 1,
            },
            DbEvent {
                message: "Toggling storage radiator".to_owned(),
                app: "git".to_owned(),
                tags: vec![1, 5],
                epoch_seconds: seconds_since_epoch + 2,
            },
        ];

        for tag in fake_tags {
            self.register_tag(tag)?;
        }

        for event in fake_events {
            self.register_event(event)?;
        }
        Ok(())
    }

    pub fn register_tag(&self, tag: DbTag) -> Result<i64> {
        self.connection.execute(
            "INSERT OR IGNORE INTO tags(text) VALUES (?1)",
            params![tag.text],
        )?;

        self.connection
            .query_row(
                "SELECT id FROM tags WHERE text = (?1)",
                params![tag.text],
                |row| {
                    let id: i64 = row.get(0)?;
                    Ok(id)
                },
            )
            .context("when retrieving ID")
    }

    pub fn register_event(&self, event: DbEvent) -> Result<()> {
        self.connection.execute(
            "INSERT INTO status(message, app, timestamp)  VALUES (?1, ?2, ?3)",
            params![event.message, event.app, event.epoch_seconds],
        )?;

        let status_id = self.connection.last_insert_rowid();

        for tag in event.tags {
            self.connection.execute(
                "INSERT INTO tags_relation(status_id, tag_id)  VALUES (?1, ?2)",
                params![status_id, tag],
            )?;
        }

        Ok(())
    }

    pub fn events_after_id(&self, start_id: i64, max_count: i64) -> Result<(i64, Vec<Event>)> {
        println!("got request for start_id: {}", start_id);
        let mut stmt = self.connection.prepare(
            r"
             SELECT status.id, message, app, timestamp, GROUP_CONCAT(text,';')
             FROM status
             INNER JOIN tags_relation ON tags_relation.status_id = status.id
             INNER JOIN tags on tags.id = tags_relation.tag_id
             WHERE status.id > ?1
             GROUP BY status.id
             ORDER BY timestamp ASC
             LIMIT ?2",
        )?;

        let max_id = std::cell::RefCell::new(start_id);
        let event_iter = stmt.query_map(params![start_id, max_count], |row| {
            let here_id: i64 = row.get(0)?;
            max_id.replace_with(|v| (*v).max(here_id));

            let tags_string: String = row.get(4)?;
            let tags = tags_string.split(";").map(|v| v.to_owned()).collect();
            Ok(Event {
                message: row.get(1)?,
                app: row.get(2)?,
                tags,
                epoch_seconds: row.get(3)?,
            })
        })?;

        let mut events = vec![];
        for event in event_iter {
            events.push(event?);
        }
        Ok((max_id.into_inner(), events))
    }

    pub fn all_tags(&self) -> Result<Vec<String>> {
        let mut stmt = self.connection.prepare(
            r"
             SELECT text FROM tags ORDER BY text",
        )?;

        let results = stmt
            .query_map(params![], |row| {
                let tag_string: String = row.get(0)?;
                Ok(tag_string)
            })?
            .collect::<Result<Vec<String>, _>>()
            .context("when reading tags")?;

        Ok(results)
    }
}
