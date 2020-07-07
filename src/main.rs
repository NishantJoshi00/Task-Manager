use clap::{App, Arg};
use std::{env, fs, fmt};
use std::path::Path;
use std::io::{Write};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
enum Outcome {
    Output(String),
    StatusCode(u8)
}

#[derive(Serialize, Deserialize)]
struct Condition {
    command: String,
    met: Outcome
}

#[derive(Serialize, Deserialize)]
struct Task {
    condition: Vec<Condition>,
    outcome: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct Instance {
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

// fn instance_from_file(fname: &String) -> Instance {
    
// }

fn main() {
    let matches = App::new("Tasker")
                    .version("0.1.0")
                    .about("A Linux Scheduling Engine for Higher Level Tasks!")
                    .author("Nishant J.")
                    .arg(Arg::with_name("dir")
                        .value_name("DIR")
                        .help("This is the working directory for the tasker"))
                    .get_matches();
    let workdir = matches.value_of("dir").unwrap_or("/usr/share/tasker");
    let workdir = Path::new(workdir);
    if !(workdir.exists()) {
        fs::create_dir_all(&workdir).unwrap();
    }
    env::set_current_dir(&workdir).unwrap();
    let build: Instance;
    if !(Path::new(&format!("{}.json", env::var("USER").unwrap())).exists()) {
        print!("Would you like to initialize the directory to be used for tasker? [Y/n] ");
        let mut string = String::new();
        std::io::stdout().flush().expect("Flush Failed");
        std::io::stdin().read_line(&mut string).ok().expect("Didn't catch that.");
        let answer = string.chars().nth(0).unwrap();
        if answer == 'Y' || answer == 'y' {
            println!("Setting up the directory for usage...");
        } else {
            std::process::exit(1)
        }

        build = Instance {
            hostname: env::var("USER").unwrap(),
            asleep: true,
            tasks: Vec::new()
        };
        
        let file = fs::File::create(Path::new(&format!("{}.json", env::var("USER").unwrap()))).expect("Unable to write");
        serde_json::ser::to_writer_pretty(file, &build).expect("Still not able to write");

    } else {
        let file = fs::File::open(Path::new(&format!("{}.json", env::var("USER").unwrap()))).expect("Unable to write");
        build = serde_json::de::from_reader(file).unwrap();
    }
    while true {
        
    }
    println!("{}", build);
}