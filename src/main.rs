use clap::{App, Arg};
use std::{env, fs};
use std::path::Path;
use std::io::{Write};

// This is a internal model for handling the process and the tasks
mod handler;
use handler::{Instance, refresh_the_build, update_disk_build, resolve_task, execute_task};
mod manager;

fn main() {

    // This is for setting up the cli for the program
    let matches = App::new("Tasker")
                    .version("0.1.0")
                    .about("A Linux Scheduling Engine for Higher Level Tasks!")
                    .author("Nishant J.")
                    .arg(Arg::with_name("dir")
                        .value_name("DIR")
                        .help("This is the working directory for the tasker"))
                    .get_matches();

    // The command line argument dir from where to pickup the work
    let workdir = matches.value_of("dir").unwrap_or("/usr/share/tasker");
    let workdir = Path::new(workdir);

    if !(workdir.exists()) {
        fs::create_dir_all(&workdir).unwrap();
    }

    // Setting the directory as `pwd`
    env::set_current_dir(&workdir).unwrap();
    
    let mut build: Instance;

    // Checking if the configuration file for a user already exists
    if !(Path::new(&format!("{}.json", env::var("USER").unwrap())).exists()) {
        // If not confering to generate the configuration file instead
        print!("Would you like to initialize the directory to be used for tasker? [Y/n] ");
        let mut string = String::new();
        std::io::stdout().flush().expect("Flush Failed");
        std::io::stdin().read_line(&mut string).ok().expect("Didn't catch that.");
        let answer = string.chars().nth(0).unwrap();
        
        // Checking the stdout that has been passed
        if answer == 'Y' || answer == 'y' {
            println!("Setting up the directory for usage...");
        } else {
            std::process::exit(1) // Exiting with a status code 1 indicating the process didn't start
        }

        // Generating the new instance for the user
        build = Instance {
            hostname: env::var("USER").unwrap(),
            asleep: true,
            tasks: Vec::new()
        };
        
        // // Opening the configuration file for the user mentioned above
        // let file = fs::File::create(Path::new(&format!("{}.json", env::var("USER").unwrap()))).expect("Unable to write");

        // // Writing the data to the file in JSON format (pretty printed)
        // serde_json::ser::to_writer_pretty(file, &build).expect("Still not able to write");
        update_disk_build(&build, Path::new(&format!("{}.json", env::var("USER").unwrap())));

    } else {
        
        // Getting the build for the already existant config file
        build = refresh_the_build(Path::new(&format!("{}.json", env::var("USER").unwrap())));
    }
    println!("{}", &build); // DEBUG: Analysing the build that is existed so far
    // GOAL: To create a function for signal handling : DONE !
    let ctrl_int = manager::signal::get_signal_abool_variable(manager::signal::signals::SIGINT).unwrap();
    let ctrl_up = manager::signal::get_signal_abool_variable(manager::signal::signals::SIGHUP).unwrap();
    loop {
        let mut i = 0;
        if ctrl_up.load(std::sync::atomic::Ordering::Relaxed) {
            build = refresh_the_build(Path::new(&format!("{}.json", env::var("USER").unwrap())));
        }
        while i < build.tasks.len() {
            if ctrl_int.load(std::sync::atomic::Ordering::Relaxed) {
                
                return;
            }
            if resolve_task(&build.tasks[i]) {
                execute_task(&build.tasks[i]);
                build.tasks.remove(i);
            } else {
                i += 1;
            }
        }
        update_disk_build(&build, Path::new(&format!("{}.json", env::var("USER").unwrap())));
        
    }
}