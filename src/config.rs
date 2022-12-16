// Copyright (C) 2021, Achiefs.
// Copyright 2022 Tintri by DDN, Inc. All rights reserved.

// Global constants definitions
pub const VERSION: &str = "0.1";
const CONFIG_PATH: &str = "/etc/ifim/config.yml";

// To parse files in yaml format
use yaml_rust::yaml::{Yaml, YamlLoader, Array};
// To use files IO operations.
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io::Write;
// To manage paths
use std::path::Path;
// To set log filter level
use simplelog::LevelFilter;
// To manage common functions
use crate::utils;
use colored::Colorize;

// ----------------------------------------------------------------------------

pub struct Config {
    pub version: String,
    pub path: String,
    pub events_file: String,
    pub monitor: Array,
    pub log_file: String,
    pub log_level: String,
    pub system: String
}

impl Config {

    pub fn clone(&self) -> Self {
        Config {
            version: self.version.clone(),
            path: self.path.clone(),
            events_file: self.events_file.clone(),
            monitor: self.monitor.clone(),
            log_file: self.log_file.clone(),
            log_level: self.log_level.clone(),
            system: self.system.clone()
        }
    }

    pub fn new(system: &str) -> Self {
        println!("{}: {}", "System detected".green(), system);
        let config_path = get_config_path();
        println!("{}: {}", "Loaded config from".green(), config_path);
        let yaml = read_config(config_path.clone());

        // Manage null value on events->file value
        let events_file = match yaml[0]["events"]["file"].as_str() {
            Some(value) => String::from(value),
            None => {
                    String::from("Not_used")
            }
        };

        // Manage null value on monitor value
        let monitor = match yaml[0]["monitor"].as_vec() {
            Some(value) => value.to_vec(),
            None => Vec::new()
        };

        // Manage null value on log->file value
        let log_file = match yaml[0]["log"]["file"].as_str() {
            Some(value) => String::from(value),
            None => {
                println!("{}", "log->file not found in config.yml.".red());
                panic!("{}", "log->file not found in config.yml.".red());
            }
        };

        // Manage null value on log->level value
        let log_level = match yaml[0]["log"]["level"].as_str() {
            Some(value) => String::from(value),
            None => {
                println!("{}", "log->level not found in config.yml, using 'info'.".yellow());
                String::from("info")
            }
        };

        Config {
            version: String::from(VERSION),
            path: config_path,
            events_file,
            monitor,
            log_file,
            log_level,
            system: String::from(system)
        }
    }

    // ------------------------------------------------------------------------

    // To process log level set on config file
    pub fn get_level_filter(&self) -> LevelFilter {
        let mut log = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(self.log_file.clone())
            .expect("(get_level_filter) Unable to open events log file.");

        match self.log_level.as_str() {
            "debug" | "Debug" | "DEBUG" | "D" | "d" => LevelFilter::Debug,
            "info" | "Info" | "INFO" | "I" | "i" => LevelFilter::Info,
            "error" | "Error" | "ERROR" | "E" | "e" => LevelFilter::Error,
            "warning" | "Warning" | "WARNING" | "W" | "w" | "warn" | "Warn" | "WARN" => LevelFilter::Warn,
            _ => {
                let msg = String::from("invalid log level from 'config.yml', using Info level.").red();
                println!("{}", msg);
                writeln!(log, "{}", msg).expect("cannot write in log file.");
                LevelFilter::Info
            }
        }
    }


    // ------------------------------------------------------------------------

    pub fn get_index(&self, raw_path: &str, cwd: &str, array: Array) -> usize {
        // Iterate over monitoring paths to match ignore string and ignore event or not
        match array.iter().position(|it| {
            if !cwd.is_empty() && (raw_path.starts_with("./") || raw_path == "." || !raw_path.contains('/')) {
                utils::match_path(cwd, it["path"].as_str().unwrap())
            }else{
                utils::match_path(raw_path, it["path"].as_str().unwrap())
            }
        }){
            Some(pos) => pos,
            None => usize::MAX
        }
    }

    // ------------------------------------------------------------------------

    pub fn get_label(&self, index: usize) -> String {
        match self.monitor[index]["label"].clone().as_str() {
            Some(label) => String::from(label),
            None => String::new()
        }
    }

    // ------------------------------------------------------------------------

    pub fn match_ignore(&self, index: usize, filename: &str, array: Array) -> bool {
        match array[index]["ignore"].as_vec() {
            Some(igv) => igv.to_vec().iter().any(|ignore| filename.contains(ignore.as_str().unwrap()) ),
            None => false
        }
    }

    // ------------------------------------------------------------------------

 /*   // Returns if a given path and filename is in the configuration paths
    pub fn path_in(&self, raw_path: &str, cwd: &str, vector: Vec<Yaml>) -> bool {
        // Iterate over monitoring paths to match ignore string and ignore event or not
        match vector.iter().any(|it| {
            if raw_path.starts_with("./") || raw_path == "." || !raw_path.contains('/') {
                utils::match_path(cwd, it["path"].as_str().unwrap())
            }else{
                utils::match_path(raw_path, it["path"].as_str().unwrap())
            }
        }){
            true => true,
            false => false
        }
    }
*/
}



// ----------------------------------------------------------------------------

// To read the Yaml configuration file
pub fn read_config(path: String) -> Vec<Yaml> {
    let mut file = File::open(path.clone())
        .unwrap_or_else(|_| panic!("(read_config): Unable to open file '{}'", path));
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    YamlLoader::load_from_str(&contents).unwrap()
}

// ----------------------------------------------------------------------------

pub fn get_config_path() -> String {
    // Select directory where to load config.yml it depends on system
    let default_path = format!("./config/config.yml");
    let relative_path = format!("./../../config/config.yml");
    if Path::new(default_path.as_str()).exists() {
        default_path
    }else if Path::new("./config.yml").exists() {
        String::from("./config.yml")
    }else if Path::new(relative_path.as_str()).exists() {
        relative_path
    }else{
        String::from(CONFIG_PATH)
    }
}

// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------

    pub fn create_test_config(filter: &str) -> Config {
        Config {
            version: String::from(VERSION),
            path: String::from("test"),
            events_file: String::from("test"),
            monitor: Array::new(),
            log_file: String::from("./test.log"),
            log_level: String::from(filter),
            system: String::from("test")
        }
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_clone() {
        let config = create_test_config("info");
        let cloned = config.clone();
        assert_eq!(config.version, cloned.version);
        assert_eq!(config.path, cloned.path);
        assert_eq!(config.events_file, cloned.events_file);
        assert_eq!(config.monitor, cloned.monitor);
        assert_eq!(config.log_file, cloned.log_file);
        assert_eq!(config.log_level, cloned.log_level);
        assert_eq!(config.system, cloned.system);
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_new_config() {
        let config = Config::new("illumos");
        assert_eq!(config.version, String::from(VERSION));
        assert_eq!(config.events_file, String::from("/var/lib/ifim/events.json"));
        // monitor
        assert_eq!(config.log_file, String::from("/var/log/ifim/ifim.log"));
        assert_eq!(config.log_level, String::from("info"));
        assert_eq!(config.system, String::from("illumos"));
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_level_filter_info() {
        let filter = LevelFilter::Info;
        assert_eq!(create_test_config("info").get_level_filter(), filter);
        assert_eq!(create_test_config("Info").get_level_filter(), filter);
        assert_eq!(create_test_config("INFO").get_level_filter(), filter);
        assert_eq!(create_test_config("I").get_level_filter(), filter);
        assert_eq!(create_test_config("i").get_level_filter(), filter);
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_level_filter_debug() {
        let filter = LevelFilter::Debug;
        assert_eq!(create_test_config("debug").get_level_filter(), filter);
        assert_eq!(create_test_config("Debug").get_level_filter(), filter);
        assert_eq!(create_test_config("DEBUG").get_level_filter(), filter);
        assert_eq!(create_test_config("D").get_level_filter(), filter);
        assert_eq!(create_test_config("d").get_level_filter(), filter);
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_level_filter_error() {
        let filter = LevelFilter::Error;
        assert_eq!(create_test_config("error").get_level_filter(), filter);
        assert_eq!(create_test_config("Error").get_level_filter(), filter);
        assert_eq!(create_test_config("ERROR").get_level_filter(), filter);
        assert_eq!(create_test_config("E").get_level_filter(), filter);
        assert_eq!(create_test_config("e").get_level_filter(), filter);
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_level_filter_warning() {
        let filter = LevelFilter::Warn;
        assert_eq!(create_test_config("warning").get_level_filter(), filter);
        assert_eq!(create_test_config("Warning").get_level_filter(), filter);
        assert_eq!(create_test_config("WARNING").get_level_filter(), filter);
        assert_eq!(create_test_config("W").get_level_filter(), filter);
        assert_eq!(create_test_config("w").get_level_filter(), filter);
        assert_eq!(create_test_config("warn").get_level_filter(), filter);
        assert_eq!(create_test_config("Warn").get_level_filter(), filter);
        assert_eq!(create_test_config("WARN").get_level_filter(), filter);
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_level_filter_bad() {
        let filter = LevelFilter::Info;
        assert_eq!(create_test_config("bad").get_level_filter(), filter);
        assert_eq!(create_test_config("BAD").get_level_filter(), filter);
        assert_eq!(create_test_config("B").get_level_filter(), filter);
        assert_eq!(create_test_config("b").get_level_filter(), filter);
        assert_eq!(create_test_config("test").get_level_filter(), filter);
        assert_eq!(create_test_config("").get_level_filter(), filter);
        assert_eq!(create_test_config("_").get_level_filter(), filter);
        assert_eq!(create_test_config("?").get_level_filter(), filter);
        assert_eq!(create_test_config("=").get_level_filter(), filter);
        assert_eq!(create_test_config("/").get_level_filter(), filter);
        assert_eq!(create_test_config(".").get_level_filter(), filter);
        assert_eq!(create_test_config(":").get_level_filter(), filter);
        assert_eq!(create_test_config(";").get_level_filter(), filter);
        assert_eq!(create_test_config("!").get_level_filter(), filter);
        assert_eq!(create_test_config("''").get_level_filter(), filter);
        assert_eq!(create_test_config("[]").get_level_filter(), filter);
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_read_config_unix() {
        let yaml = read_config(String::from("config/config.yml"));

        assert_eq!(yaml[0]["events"]["file"].as_str().unwrap(), "/var/lib/ifim/events.json");

        assert_eq!(yaml[0]["monitor"][0]["path"].as_str().unwrap(), "/bin");
        assert_eq!(yaml[0]["monitor"][1]["path"].as_str().unwrap(), "/usr/bin");
        assert_eq!(yaml[0]["monitor"][1]["label"].as_str().unwrap(), "usr/bin");

        assert_eq!(yaml[0]["log"]["file"].as_str().unwrap(), "/var/log/ifim/ifim.log");
        assert_eq!(yaml[0]["log"]["level"].as_str().unwrap(), "info");
    }

    // ------------------------------------------------------------------------

    #[test]
    #[should_panic(expected = "NotFound")]
    fn test_read_config_panic() {
        read_config(String::from("NotFound"));
    }

    // ------------------------------------------------------------------------

    #[test]
    #[should_panic(expected = "ScanError")]
    fn test_read_config_panic_not_config() {
        read_config(String::from("README.md"));
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_config_path() {
        let default_path = "./config/config.yml";
        assert_eq!(get_config_path(), default_path);
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_index() {
        let config = Config::new(&utils::get_os());
        if utils::get_os() == "illumos" {
            assert_eq!(config.get_index("/bin/", "", config.monitor.clone()), 0);
            assert_eq!(config.get_index("./", "/bin", config.monitor.clone()), 0);
            assert_eq!(config.get_index("/usr/bin/", "", config.monitor.clone()), 1);
            assert_eq!(config.get_index("/etc", "", config.monitor.clone()), 2);
            assert_eq!(config.get_index("/test", "", config.monitor.clone()), usize::MAX);
            assert_eq!(config.get_index("./", "/test", config.monitor.clone()), usize::MAX);
        }
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_label() {
        let config = Config::new(&utils::get_os());
        let label = config.get_label(config.get_index("/usr/bin","", config.monitor.clone()));
        assert_eq!(label, "usr/bin");
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_match_ignore() {
        let config = Config::new(&utils::get_os());
        assert!(config.match_ignore(0, "file.swp", config.monitor.clone()));
        assert!(!config.match_ignore(0, "file.txt", config.monitor.clone()));
    }

}
