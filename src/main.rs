// Modules

pub mod data;
pub mod interface;

// Dependencies

use crate::interface::Gui;

// Program entry

fn main() {
	let options = eframe::NativeOptions::default();
	eframe::run_native(
		"Plugin",
		options,
		Box::new(|_cc| Box::new(Gui::new()))
	);
}