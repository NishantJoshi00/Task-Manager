use clap::{App, Arg};
use std::{env, fs, fmt};
use std::path::Path;
use std::io::{Write};
use serde::{Serialize, Deserialize};
use serde_json;

// This is a internal model for handling the process and the tasks
mod handler;

#[derive(Serialize, Deserialize)]
enum Outcome {
    /*
    Structure based enum for outcome
    This is for the condition fulfillment based on one of two types
    - The stdout that is expected
    - The return status code of the command that is to be executed
    */
    Output(String),
    StatusCode(u8)
}

#[derive(Serialize, Deserialize)]
struct Condition {
    /*
    A structure for holding the condition clause for the Task
    It holds the command that is to be executed in String format
    As well as the expected outcome in hit
    */
    command: String,
    hit: Outcome
}

#[derive(Serialize, Deserialize)]
struct Task {
    /*
    This structure holds details for a task that is to be executed
    - condition: It holds all the conditions that are to be met simultaneously
        In form of a resizible vector of struct Condition
    - outcome: This holds all the commands that are to be executed when the condition is met
    */
    condition: Vec<Condition>,
    outcome: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct Instance {
    /*
    Structure holding the detail of the configuration for a specific user
    - It has the hostname of the user
    - The state tasker is in
        This deternimes if it is looking for any tasks
    - The List of all the tasks assigned to it.
    */
    hostname: String,
    asleep: bool,
    tasks: Vec<Task>,
}

impl fmt::Display for Instance {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", serde_json::ser::to_string_pretty(&self).unwrap())
    }
}

fn refresh_the_build(p: &Path) -> Instance {
    // Function for refreshing the build for the JSON config file
    // The file that is opened as read-only
    let file = fs::File::open(&p).expect("Unable to write");
    // Returning the parsed JSON for the file as Instance
    serde_json::de::from_reader(file).unwrap()
}

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
    
    let build: Instance;

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
        
        // Opening the configuration file for the user mentioned above
        let file = fs::File::create(Path::new(&format!("{}.json", env::var("USER").unwrap()))).expect("Unable to write");

        // Writing the data to the file in JSON format (pretty printed)
        serde_json::ser::to_writer_pretty(file, &build).expect("Still not able to write");

    } else {
        
        // Getting the build for the already existant config file
        build = refresh_the_build(Path::new(&format!("{}.json", env::var("USER").unwrap())))
    }
    println!("{}", &build); // DEBUG: Analysing the build that is existed so far


    // GOAL: To create a new process to handle the tasks that are present in the build <- variable
    // GOAL: To create a function for signal handling
    // GOAL: To create a function to store the updated config file in the config file
}