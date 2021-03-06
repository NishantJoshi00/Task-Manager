use std::fmt;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub enum Outcome {
    /*
    Structure based enum for outcome
    This is for the condition fulfillment based on one of two types
    - The stdout that is expected
    - The return status code of the command that is to be executed
    */
    Output(String),
    StatusCode(i32)
}

#[derive(Serialize, Deserialize)]
pub struct Condition {
    /*
    A structure for holding the condition clause for the Task
    It holds the command that is to be executed in String format
    As well as the expected outcome in hit
    */
    pub command: String,
	pub hit: Outcome
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    /*
    This structure holds details for a task that is to be executed
    - condition: It holds all the conditions that are to be met simultaneously
        In form of a resizible vector of struct Condition
    - outcome: This holds all the commands that are to be executed when the condition is met
    */
    pub condition: Vec<Condition>,
    pub outcome: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Instance {
    /*
    Structure holding the detail of the configuration for a specific user
    - It has the hostname of the user
    - The state tasker is in
        This deternimes if it is looking for any tasks
    - The List of all the tasks assigned to it.
    */
    pub hostname: String,
    pub asleep: bool,
    pub tasks: Vec<Task>,
}

// Impls for Outcome
impl fmt::Display for Outcome {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            Outcome::Output(e) => write!(f, "Output(\"{}\")", e),
            Outcome::StatusCode(e) => write!(f, "StatusCode({})", e)
        }
    }
}

impl Clone for Outcome {
    fn clone(&self) -> Self {
        match self {
            Outcome::Output(stri) => Outcome::Output(stri.clone()),
            Outcome::StatusCode(int) => Outcome::StatusCode(int.clone())
        }
    }
}


// Impls for Condition
impl fmt::Display for Condition {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Condition {{ command: \"{}\", expected: {} }}", self.command, self.hit)
    }
}

impl Clone for Condition {
    fn clone(&self) -> Self {
        return Self {
            command: self.command.clone(),
            hit: self.hit.clone()
        }
    }
}


// Impls for Tasks
impl fmt::Display for Task {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Outcome:\n")?;
        write!(f, "conditions: \n")?;
        for i in &self.condition {
            write!(f, "\t- {}\n", i)?;
        }
        write!(f, "outcomes: \n")?;
        for i in &self.outcome {
            write!(f, "\t- {}\n", i)?;
        }
        write!(f, "")
    }
}

impl Clone for Task {
    fn clone(&self) -> Self {
        return Self {
            condition: self.condition.clone(),
            outcome: self.outcome.clone()
        }
    }
}



// Impls for Instance

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