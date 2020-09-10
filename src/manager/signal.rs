use signal_hook;

use std::sync::{Arc, atomic::AtomicBool};

pub mod signals {
	pub use signal_hook::{SIGINT, SIGTERM, SIGHUP, SIGCONT};
}

pub fn get_signal_abool_variable(sig: i32) -> Result<Arc<AtomicBool>, std::io::Error> {
	let int = Arc::new(AtomicBool::new(false));
	signal_hook::flag::register(sig, Arc::clone(&int))?;
	Ok(int)
}
