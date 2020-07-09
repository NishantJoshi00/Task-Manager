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
	let mut e: Vec<&str> = s.split_whitespace().collect();
	let cmd = e.remove(0).to_owned();
	let mut args: Vec<String> = Vec::new();
	for x in e {
		args.push(x.to_owned());
	}
	(cmd, args)
}

fn vec_to_string(e: Vec<u8>) -> String {
	let mut string = String::new();
	for i in e {
		string.push(i as char);
	}
	string
}


pub fn resolve_condition(c: &Condition) -> bool {
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

pub fn resolve_task(t: &Task) -> bool {
	let mut dec: bool = true;
	for c in &t.condition {
		dec = dec & resolve_condition(c);
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