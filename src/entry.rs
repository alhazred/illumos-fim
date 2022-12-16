// Copyright (C) 2021, Achiefs.
// Copyright 2022 Tintri by DDN, Inc. All rights reserved.

// To implement Debug and fmt method
use std::fmt;
// To handle files
use std::fs::OpenOptions;
use std::io::Write;
//use std::io::{Write, Error, ErrorKind};
// Event handling
// To log the program procedure
use log::*;
// To handle JSON objects
use serde_json::{json, to_string};
// To manage paths

use notify::event::EventKind;

// To get configuration constants
use crate::config;

pub struct Entry {
    pub id: String,
    pub path: String,
    pub mode: String,
    pub uid: String,
    pub gid: String,
    pub filesize: String,
    pub mtime: String,
    pub atime: String,
    pub ctime: String,
    pub operation: String,
    pub timestamp: String,
    pub checksum: String,
    pub label: String
}

impl Entry {
    // Get formatted string with all required data
    fn format_json(&self) -> String {
        let obj = json!({
            "id": self.id.clone(),
            "path": self.path.clone(),
            "mode": self.mode.clone(),
            "uid": self.uid.clone(),
            "gid": self.gid.clone(),
            "filesize": self.filesize.clone(),
            "mtime": self.mtime.clone(),
            "atime": self.atime.clone(),
            "ctime": self.ctime.clone(),
            "operation": self.operation.clone(),
            "timestamp": self.timestamp.clone(),
            "checksum": self.checksum.clone(),
            "label": self.label.clone()
        });
        to_string(&obj).unwrap()
    }

    // ------------------------------------------------------------------------

    // Function to write the received events to file
    pub fn log(&self, file: String){
        let mut events_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file)
            .expect("(log) Unable to open events log file.");

             match writeln!(events_file, "{}", self.format_json() ) {
                    Ok(_d) => debug!("Event log written"),
                    Err(e) => error!("Event could not be written, Err: [{}]", e)
                };
    }

    // ------------------------------------------------------------------------

    // Function to manage event destination
    pub async fn process(&self, config: config::Config){
         self.log(config.events_file);
    }
}

pub struct Rentry {
    pub id: String,
    pub path: String,
    pub operation: String,
    pub timestamp: String,
    pub label: String
}

impl Rentry {
    // Get formatted string with all required data
    fn format_json(&self) -> String {
        let obj = json!({
            "id": self.id.clone(),
            "path": self.path.clone(),
            "operation": self.operation.clone(),
            "timestamp": self.timestamp.clone(),
            "label": self.label.clone()
        });
        to_string(&obj).unwrap()
    }

    // ------------------------------------------------------------------------

    // Function to write the received events to file
    pub fn log(&self, file: String){
        let mut events_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file)
            .expect("(log) Unable to open events log file.");

             match writeln!(events_file, "{}", self.format_json() ) {
                    Ok(_d) => debug!("Event log written"),
                    Err(e) => error!("Event could not be written, Err: [{}]", e)
                };
    }

    // ------------------------------------------------------------------------

    // Function to manage event destination
    pub async fn process(&self, config: config::Config){
         self.log(config.events_file);
    }
}


// ----------------------------------------------------------------------------

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        f.debug_tuple("")
          .field(&self.id)
          .field(&self.path)
          //.field(&self.operation)
          .finish()
    }
}

// ----------------------------------------------------------------------------
pub async fn parse_event(event: notify::event::Event) -> String {
    match event.kind {
        EventKind::Create(_) =>  { String::from("CREATE") },
        EventKind::Remove(_) =>  { String::from("REMOVE") },
        EventKind::Modify(_) =>  { String::from("MODIFY") },
        EventKind::Access(_) =>  { String::from("ACCESS") },
        EventKind::Any => { String::from("UNKNOWN") },
        EventKind::Other =>  { String::from("UNKNOWN") }
    }
}

// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::entry::Entry;
    use std::fs;

    // ------------------------------------------------------------------------

    fn remove_test_file(filename: String) {
        fs::remove_file(filename).unwrap()
    }

    fn create_test_entry() -> Entry {
        Entry {
            id: "Test_id".to_string(),
            path: "/home/user".to_string(),
            mode: "100644".to_string(),
            uid: "100".to_string(),
            gid: "100".to_string(),
            filesize: "100".to_string(),
            mtime: "Timestamp".to_string(),
            atime: "Timestamp".to_string(),
            ctime: "Timestamp".to_string(),
            operation: "TEST".to_string(),
            timestamp: "Timestamp".to_string(),
            checksum: "UNKNOWN".to_string(),
            label: "test".to_string()
        }
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_create_entry() {
        let evt = create_test_entry();
        assert_eq!(evt.id, "Test_id".to_string());
        assert_eq!(evt.path, "/home/user".to_string());
        assert_eq!(evt.mode, "100644".to_string());
        assert_eq!(evt.uid, "100".to_string());
        assert_eq!(evt.gid, "100".to_string());
        assert_eq!(evt.filesize, "100".to_string());
        assert_eq!(evt.mtime, "Timestamp".to_string());
        assert_eq!(evt.atime, "Timestamp".to_string());
        assert_eq!(evt.ctime, "Timestamp".to_string());
        assert_eq!(evt.operation, "TEST".to_string());
        assert_eq!(evt.timestamp, "Timestamp".to_string());
        assert_eq!(evt.checksum, "UNKNOWN".to_string());
        assert_eq!(evt.label, "test".to_string());
     
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_entry_fmt(){
        let out = format!("{:?}", create_test_entry());
        assert_eq!(out, "(\"Test_id\", \"/home/user\")");
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_format_json() {
        let expected = "{\"id\":\"Test_id\",\"path\":\"/home/user\",\"mode\":\"100644\",\
            \"uid\":\"100\",\"gid\":\"100\",\"filesize\":\"100\",\
            \"mtime\":\"Timestamp\",\"atime\":\"Timestamp\",\"ctime\":\"Timestamp\",\
            \"operation\":\"TEST\",\"timestamp\":\"Timestamp\",\"checksum\":\"UNKNOWN\",\"label\":\"test\"}";
        assert_eq!(create_test_entry().format_json(), expected);
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_log() {
        let filename = String::from("test_entry.json");
        let evt = create_test_entry();

        evt.log(filename.clone());
        let contents = fs::read_to_string(filename.clone());
        let expected = "{\"id\":\"Test_id\",\"path\":\"/home/user\",\"mode\":\"100644\",\
            \"uid\":\"100\",\"gid\":\"100\",\"filesize\":\"100\",\
            \"mtime\":\"Timestamp\",\"atime\":\"Timestamp\",\"ctime\":\"Timestamp\",\
            \"operation\":\"TEST\",\"timestamp\":\"Timestamp\",\"checksum\":\"UNKNOWN\",\"label\":\"test\"}\n";
        assert_eq!(contents.unwrap(), expected);
        remove_test_file(filename.clone());
    }
}
