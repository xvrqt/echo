/* Standard Library */
use std::result::Result;
use std::process::{Command, Stdio};
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::path::{Path, PathBuf};
use std::env::temp_dir;
use std::fs;

/* Third Party Libraries */
use clap::ArgMatches;
use user_error::UserError;

/* Internal Modules */
use crate::db;
use crate::utility;

/* Constants used by the struct to prevent naming collisions of buffer */
const PREFIX: &str = "xvrqt_echo";
static TEMP_FILE_COUNT: AtomicUsize = AtomicUsize::new(0);

/* Creates a thread safe, process safe tempfile to use as a buffer */
fn create_temp_file() -> Result<PathBuf, UserError> {
    /* Generate unique path to a temporary file buffer */
    let process_id = std::process::id();
    let i = TEMP_FILE_COUNT.fetch_add(1, Ordering::SeqCst);
    let ext = "txt";
    let temp_file = format!("{}_{}_{}{}", PREFIX, process_id, i, ext);

    /* Push the file to the OS's temp dir */
    let mut temp_dir = temp_dir();
    temp_dir.push(temp_file);

    /* Create the file */
    fs::File::create(&temp_dir).map_err(|_e| {
        UserError::simple("Failed to create tempfile")
    })?;

    Ok(temp_dir)
}


/* Lists summarized versions of each post to less */
pub fn run(args: &ArgMatches) -> Result<(), UserError> {
    /* Get the project path to build */
    let (config, connection) = utility::get_project(args.value_of("PATH"))?;

    /* Grab all the posts */
    let posts = db::get_latest(&connection, 1000)?;
    let num_posts = db::num_posts(&connection)?;

    /* Flatten vectors */
    let mut hold = Vec::new();
    for post_vec in posts {
        for post in post_vec {
            hold.push(post);
        }
    }

    /* Transform them all into a summary and assign an ID */
    let mut count = 0;
    let log: Vec<String> = hold.iter().map(|p| { 
        let id = num_posts - count;
        count = count + 1;
        let summary: String = p.text.chars().take(50).collect();
        format!("{}: {}", id, summary)
    }).collect();

    let posts = log.join("\n");
    let buffer = create_temp_file()?;
    fs::write(&buffer, posts).map_err(|_| {
        UserError::simple("Failed to write to tempfile")
    })?;

    Command::new("less")
            .arg(&buffer)
            .status()
            .expect("Failed to open 'less'");

    Ok(())
}

