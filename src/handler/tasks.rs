use std::{path::Path, fs};
use std::process::{Command, Stdio};
use super::datatypes::{Instance, Condition, Outcome, Task};

pub fn refresh_the_build(p: &Path) -> Instance {
    // Function for refreshing the build for the JSON config file
    // The file that is opened as read-only
    let file = fs::File::open(&p).expect("Unable to write");
    // Returning the parsed JSON for the file as Instance
    serde_json::de::from_reader(file).unwrap()
}



fn split_string_for_command(s: &String) -> (String, Vec<String>) {
	/*
	This function converts a continuous string into a tuple
		This function is specifically made for converting a continuous command into
		- This base command in type: String
		- The args that are passed to the command
	*/
	let mut e: Vec<&str> = s.split_whitespace().collect();
	let cmd = e.remove(0).to_owned();
	let mut args: Vec<String> = Vec::new();
	for x in e {
		args.push(x.to_owned());
	}
	(cmd, args)
}

fn vec_to_string(e: Vec<u8>) -> String {
	/*
	This function converts an vector of 1byte integers 
		into character and builds a string from them
	*/
	let mut string = String::new();
	for i in e {
		string.push(i as char);
	}
	string
}


pub fn resolve_condition(c: &Condition) -> bool {
	/*
	This function borrows a condition struct
		- Checks if the command specified runs
		- Compares it with the enum (Outcome)
			- If it is output: Then it does a string compare between the expected output
				and the output recieved from the command
			- If it is StatusCode: Then it dumps the output and compares the status code
				of the command to the expected value
		- Then returns bool representing that the condition was fulfilled or not
	*/
	let command = split_string_for_command(&c.command);
	match &c.hit {
		Outcome::Output(op) => {
			if command.1.len() == 0 {
				let outcome = Command::new(command.0)
					.output()
					.unwrap().stdout;
				if op == &vec_to_string(outcome) {
					return true;
				}
			} else {
				let outcome = Command::new(command.0)
					.args(command.1)
					.output()
					.unwrap().stdout;
				if op == &vec_to_string(outcome) {
					return true;
				}
			}
		}
		Outcome::StatusCode(sc) => {
			if command.1.len() == 0 {
				let outcome = Command::new(command.0)
					.stdout(Stdio::null())
					.status()
					.unwrap();
				if &outcome.code().unwrap() == sc {
					return true;
				}
			} else {
				let outcome = Command::new(command.0)
					.args(command.1)
					.stdout(Stdio::null())
					.status()
					.unwrap();
				if &outcome.code().unwrap() == sc {
					return true;
				}
			}
		}
	}

	false
}



#[allow(dead_code)]
pub fn resolve_task(t: &Task) -> bool {
	/*
	This function borrows a Task and sees if all the condition that are present in the clause
		Are true or not
		- If true: Then It runs the commands that are expected to run to fulfill the task

	*/
	let mut dec: bool = true;
	for c in &t.condition {
		dec = dec & resolve_condition(c); // The serial way to do it
	}

	if dec {
		for oc in &t.outcome {
			let cmd = split_string_for_command(oc);
			if cmd.1.len() == 0 {
				Command::new(cmd.0)
					.stdout(Stdio::null())
					.status()
					.unwrap();
			} else {
				Command::new(cmd.0)
					.args(cmd.1)
					.stdout(Stdio::null())
					.status()
					.unwrap();
			}
		}
		return true;
	}

	return false;
}