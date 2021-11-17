// Note   Example Code from: https://stackoverflow.com/questions/55440289/how-do-i-recursively-watch-file-changes-in-rust
// Note   crfate notify api doc: https://docs.rs/notify/4.0.10/notify/#default-debounced-api
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

use chrono::prelude::*;
// use chrono::prelude::{DateTime, Utc};

const MY_FOLDER_TO_WATCH: &str = "c:/tmp/__test";
const MY_REFRESH_RATE_FOR_WATCHING_IN_SECONDS: u64 = 5;
static MY_PROGRAM_NAME: & str = "folderwatch0002";

fn iso8601now() -> String {
    // Note formats like "2021-11-18T23-34-60.026490+09:30"
    // Note instead of : we use - between  the time parts 
    // Note so we canuse the string for directory and filenames
    let local: DateTime<Local> = Local::now();
   // Note .format(%f) < %f = nanoseconds
    // format!("{}", local.format("%Y-%m-%dT%H-%M-%S.%f%z").to_string()) 
    // Removed timezone (%z) from the format string because we don't need it
    format!("{}", local.format("%Y-%m-%dT%H-%M-%S.%f").to_string()) 
}
    

fn main() {
    // let st = SystemTime::now();
    // let local: DateTime<Local> = Local::now();
    // let strlocal: String = iso8601now();
    
    // println!("Started: {}", local);
    // println!("=================================================================");
    println!("===================================================[{}]", MY_PROGRAM_NAME);
    // println!("Started         : {} [{}]", iso8601now(), MY_PROGRAM_NAME);
    println!("Started       : {}", iso8601now());
    println!("Refreshrate is: {} seconds", MY_REFRESH_RATE_FOR_WATCHING_IN_SECONDS);
    println!("--------------------------------------------------------------------");
    println!("");
    
    // println!("{}", systemtime_strftime(st, "%d/%m/%Y %T"));

    // Create a channel to receive the events.
    let (sender, receiver) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    // let mut watcher = watcher(sender, Duration::from_secs(10)).unwrap();
    let mut watcher = watcher(sender, Duration::from_secs(MY_REFRESH_RATE_FOR_WATCHING_IN_SECONDS)).unwrap();
        

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(MY_FOLDER_TO_WATCH, RecursiveMode::Recursive).unwrap();

    loop {
        match receiver.recv() {
            // Ok(event) => println!("{:?}", event),
            Ok(event) => {
                // let local: DateTime<Local> = Local::now();
                
                println!("{} -- {:?}", iso8601now(), event);
            },
           Err(e) => println!("watch error: {:?}", e),
        }
    }
}

