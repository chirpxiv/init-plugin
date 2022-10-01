// Dependencies

use egui::Ui;
use eframe::egui;

// Current tab

#[derive(PartialEq)]
pub enum Tab {
	Basics,
	Dependencies
}

// Gui

pub struct Gui {
	pub tab: Tab
}

impl Gui {
	pub fn new() -> Self {
		Self {
			tab: Tab::Basics
		}
	}

	// Tabs

	fn tab_basics(&mut self, ui: &mut Ui) {
		
	}
}

impl eframe::App for Gui {
	// Main draw function

	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		// Side panel

		egui::SidePanel::left("side_left")
		.show(ctx, |ui| {
			// Tabs
			// TODO: Streamline this.

			if ui.selectable_label(self.tab == Tab::Basics, "Basics").clicked()
				{ self.tab = Tab::Basics; }
			if ui.selectable_label(self.tab == Tab::Dependencies, "Dependencies").clicked()
				{ self.tab = Tab::Dependencies; }
		});

		// Draw tab

		egui::CentralPanel::default().show(ctx, |ui| {
			match self.tab {
				Tab::Basics => self.tab_basics(ui),
				_ => {}
			}
		});
	}

}