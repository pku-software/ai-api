mod student;

use csv;
use rusqlite::{params, Connection, Result};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Mutex;

use student::Student;

use crate::config::Config;

pub(crate) struct SQLiteDatebase {
    pub(crate) conn: Connection,
    mutex: Mutex<()>,
}

impl SQLiteDatebase {
    pub fn new(config: &Config) -> Result<SQLiteDatebase> {
        let conn = Connection::open(&config.db_file)?;
        let mutex = Mutex::new(());

        Ok(SQLiteDatebase { conn, mutex })
    }

    pub fn init(&self, config: &Config) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS students (
                  id              TEXT PRIMARY KEY,
                  num             INTEGER NOT NULL
                  )",
            params![],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS version (
                  hash            TEXT PRIMARY KEY
                  )",
            params![],
        )?;

        // generate current version
        let mut hasher = Sha256::new();
        let path = Path::new(&config.user_csv);
        let file = File::open(path);
        if file.is_err() {
            error!("User csv file not found: {}", config.user_csv);
            return Err(rusqlite::Error::InvalidQuery);
        }
        let mut file = file.unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        hasher.update(buffer);
        let current_hash = format!("{:x}", hasher.finalize());

        // read version
        let mut stmt = self.conn.prepare("SELECT hash FROM version")?;
        let mut rows = stmt.query(params![])?;

        // If version is not exist, insert it
        if let Ok(Some(row)) = rows.next() {
            let hash: String = row.get(0)?;
            if hash != current_hash {
                info!("DB version is not match, updating...");
                self.regenerate(config)?;
            }
        } else {
            info!("DB version is not exist, updating...");
            self.regenerate(config)?;
        }

        // update version
        self.conn.execute("DELETE FROM version", params![])?;
        self.conn.execute(
            "INSERT INTO version (hash) VALUES (?)",
            params![current_hash],
        )?;

        Ok(())
    }

    fn regenerate(&self, config: &Config) -> Result<()> {
        // convert first colomn to set of String
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(&config.user_csv)
            .unwrap();

        let mut set = BTreeSet::new();

        for result in rdr.records() {
            let record = result.unwrap();
            set.insert(record.get(0).unwrap().to_string());
        }

        // iterate db, for each student, if not in set, delete it, and delete from set
        let mut stmt = self.conn.prepare("SELECT id, num FROM students")?;
        let mut rows = stmt.query(params![])?;

        while let Some(row) = rows.next()? {
            let stu: Student = Student {
                id: row.get(0).unwrap(),
                num: row.get(1).unwrap(),
            };
            if set.contains(&stu.id) {
                set.remove(&stu.id);
            } else {
                info!("Deleting student: {}", stu.id);
                self.conn
                    .execute("DELETE FROM students WHERE id = ?", params![stu.id])?;
            }
        }

        // add remain students in set to db
        for stu in set {
            info!("Adding student: {}", stu);
            self.conn.execute(
                "INSERT INTO students (id, num) VALUES (?, ?)",
                params![stu, 0],
            )?;
        }
        Ok(())
    }

    fn get_student(&self, id: &str) -> Result<Student> {
        let _lock = self.mutex.lock().unwrap();
        let mut stmt = self
            .conn
            .prepare("SELECT id, num FROM students WHERE id = ?")?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let stu: Student = Student {
                id: row.get(0).unwrap(),
                num: row.get(1).unwrap(),
            };
            Ok(stu)
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    pub fn get_num(&self, id: &str) -> Result<i32> {
        let stu = self.get_student(id)?;
        Ok(stu.num)
    }

    // thread safe add num
    pub fn add_num(&self, id: &str, num: i32) -> Result<()> {
        let _lock = self.mutex.lock().unwrap();
        let stu = self.get_student(id)?;
        self.conn.execute(
            "UPDATE students SET num = ? WHERE id = ?",
            params![stu.num + num, id],
        )?;
        Ok(())
    }
}
