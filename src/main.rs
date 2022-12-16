// Copyright (C) 2021, Achiefs.
// Copyright 2022 Tintri by DDN, Inc. All rights reserved. 

// To allow big structs like json on audit events
#![recursion_limit = "256"]

// To read and write directories and files
use std::fs;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};
// To log the program process
use log::{info, error, debug};
use simplelog::{WriteLogger}; //, Config as SimpleConfig};
// To manage paths
use std::path::Path;
use std::fs::metadata;
use std::os::unix::fs::FileTypeExt;
// To manage date and time
use std::time::{SystemTime, UNIX_EPOCH};
// To use intersperse()
use itertools::Itertools;
// Colorize
use colored::Colorize;
// Utils functions
mod utils;
// Hashing functions
mod hash;
// Configuration load functions
mod config;
// Single event data management
mod entry;

// ----------------------------------------------------------------------------

fn setup_logger(config: config::Config){
    // Create folders to store logs based on config.yml
    fs::create_dir_all(Path::new(&config.log_file).parent().unwrap().to_str().unwrap()).unwrap();

    let log_config = simplelog::ConfigBuilder::new()
        .set_time_format_custom(simplelog::format_description!(
            "[month repr:short] [day] [hour]:[minute]:[second]"
        ))
        .build();
    
    // Create logger output to write generated logs.
    WriteLogger::init(
        config.get_level_filter(),
        log_config,
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(config.log_file)
            .expect("Unable to open log file")
    ).unwrap();
}

// ----------------------------------------------------------------------------

fn setup_events(config: config::Config){
    // Perform actions depending on destination
    info!("Events file: {}", config.events_file);
    fs::create_dir_all(Path::new(&config.events_file).parent().unwrap().to_str().unwrap()).unwrap()
}

// ----------------------------------------------------------------------------

// Main function where the magic happens
#[tokio::main]
async fn main() {
    println!("{}", "Reading config...".green());
    let config = config::Config::new(&utils::get_os());
    println!("{}: {}", "Log file".green(), config.log_file);
    println!("{}: {}", "Log level".green(), config.log_level);

    setup_logger(config.clone());
    setup_events(config.clone());

    info!("illumos File Integrity Monitor started");
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default()).unwrap();

    if ! config.monitor.is_empty() {
        for element in config.monitor.clone() {
            let path = element["path"].as_str().unwrap();
            info!("Monitoring path: {}", path);
            match element["ignore"].as_vec() {
                Some(ig) => {
                    let ignore_list_vec  = ig.iter().map(|e| { e.as_str().unwrap() });
                    let ignore_list : String = Itertools::intersperse(ignore_list_vec, ", ").collect();
                    info!("Ignoring files with: {} inside {}", ignore_list, path);
                },
                None => info!("Ignore for '{}' not set", path)
            };
            match watcher.watch(path.as_ref(), RecursiveMode::Recursive) {
                Ok(_d) => debug!("Monitoring path: {}", path),
                Err(e) => println!("{} {}", "Could not monitor given path".red(), e)
            };
        }
    }
    for res in rx {
        match res {
            Ok(event) => {
                debug!("Event received: {:?}", event);
                let plain_path = event.paths[0].display().to_string();
                let event_path = Path::new(&plain_path);
                let event_filename = event_path.file_name().unwrap().to_str().unwrap();

                let path = event.paths[0].display().to_string();
                let current_timestamp = format!("{:?}", SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis());
             
                if event_path.exists() {
                    let ftype = metadata(event_path).unwrap().file_type();
                    if ftype.is_fifo() || ftype.is_block_device() || ftype.is_char_device() || ftype.is_socket() { continue; } 
                    if ftype.is_dir() { continue; }
                }

                let index = config.get_index(event_path.to_str().unwrap(), "", config.monitor.clone().to_vec());
                if index != usize::MAX {
                    let label = config.get_label(index);
                    if ! config.match_ignore(index,event_filename, config.monitor.clone()) {
                        if ! event.kind.is_remove() {
                            let entry = entry::Entry {
                                id: utils::get_uuid(),
                                path: utils::get_path(event_path),
                                mode: utils::get_perms(event_path),
                                uid: utils::get_uid(event_path),
                                gid: utils::get_gid(event_path),
                                filesize: utils::get_size(event_path), 
                                mtime: utils::get_mtime(event_path),
                                atime: utils::get_atime(event_path),
                                ctime: utils::get_ctime(event_path),
                                operation: entry::parse_event(event.clone()).await,
                                timestamp: current_timestamp,
                                label,
                                checksum: hash::get_checksum(path.to_string())
                            };
                            entry.process(config.clone()).await;  
                            info!("Changes found: {} {}", plain_path, entry.operation);  
                        } else {
                            let entry = entry::Rentry {
                                id: utils::get_uuid(),
                                path: utils::get_path(event_path),
                                operation: entry::parse_event(event.clone()).await,
                                timestamp: current_timestamp,
                                label
                            };
                            entry.process(config.clone()).await;  
                            info!("Changes found: {} {}", plain_path, entry.operation);  
                        }
                        debug!("Event processed: {:?}", event);
                    }
                }
            },
            Err(e) => {
                error!("watch error: {:?}", e);
            }
        };
    }
}

