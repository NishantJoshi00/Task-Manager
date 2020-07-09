use std::{path::Path, fs};
use super::datatypes::{Instance, Condition, Outcome};

pub fn refresh_the_build(p: &Path) -> Instance {
    // Function for refreshing the build for the JSON config file
    // The file that is opened as read-only
    let file = fs::File::open(&p).expect("Unable to write");
    // Returning the parsed JSON for the file as Instance
    serde_json::de::from_reader(file).unwrap()
}


pub fn split_string_for_command(s: String) -> (String, Vec<String>) {
	let mut e: Vec<&str> = s.split_whitespace().collect();
	let cmd = e.remove(0).to_owned();
	let mut args: Vec<String> = Vec::new();
	for x in e {
		args.push(x.to_owned());
	}
	(cmd, args)
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn resolve_condition(c: &Condition) -> bool {

	match &c.hit {
		Outcome::Output(op) => {
			
		}
		Outcome::StatusCode(sc) => {

		}
	}

	false
}