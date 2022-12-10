use std::env::var;
use std::fs::read_dir;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
#[command(name = "Firefox - Profile Transfer Tool")]
#[command(author = "Sharath <sharathdev99@gmail.com>")]
#[command(version = "0.0.1")]
#[command(about = "Transfers existing Firefox profiles to a new Firefox download", long_about=None)]
struct Cli {
    // Argument - directory containing all profiles to be transfered
    profiles_directory: PathBuf,
}

fn get_firefox_profiles_path() -> Result<PathBuf> {
    let mut firefox_profiles_path: PathBuf = PathBuf::new();
    if cfg!(target_os = "windows") {
        let app_data = var("APPDATA").with_context(|| format!("APPDATA"))?;
        firefox_profiles_path = PathBuf::from(app_data + r"\Mozilla\Firefox\");
    } else if cfg!(target_os = "linux") {
        let home = var("HOME").with_context(|| format!("HOME"))?;
        firefox_profiles_path = PathBuf::from(home + r"~/.mozilla/firefox/")
    } else if cfg!(target_os = "macos") {
        let home = var("HOME").with_context(|| format!("HOME"))?;
        firefox_profiles_path = PathBuf::from(home + r"~/Library/Application Support/Firefox/");
    }
    return Ok(firefox_profiles_path);
}

fn main() -> Result<()> {
    // Set path of Firefox profiles depending on OS
    let firefox_profiles_path: PathBuf =
        get_firefox_profiles_path().with_context(|| format!("Unable to find directory"))?;
    println!("{:?}", &firefox_profiles_path);

    // Parse CLI arguments
    let args = Cli::parse();

    // Gets an iterator over the directories present in the given directory
    println!("Profiles are located in: {:?}", &args.profiles_directory);
    let read_profiles_directory = read_dir(&args.profiles_directory)
        .with_context(|| format!("Could not read directory {:?}", &args.profiles_directory))?;

    // Iterates through the iterator and maps the paths
    let profiles_directories = read_profiles_directory
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    // Iterates through the directories and returns the directory name alone
    println!("Directories: ");
    for profile_directory in profiles_directories {
        if profile_directory.is_dir() {
            let profile_name_os_string = profile_directory.iter().last().with_context(|| {
                format!(
                    "Could not read directory name {:?}",
                    &profile_directory.display()
                )
            })?;
            let profile_name = profile_name_os_string.to_str().unwrap();
            println!("{}", profile_name)
        }
    }

    // TODO: OPEN CONFIG FILE

    // TODO: WRITE FILENAMES TO CONFIG FILE

    // TODO: COPY DIRECTORIES TO PROFILE DIRECTORY
    Ok(())
}
