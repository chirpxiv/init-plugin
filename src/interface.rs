// Dependencies

use egui::{Ui, RichText};
use eframe::egui;

use crate::data::Service;

// Constants

// TODO: Read local directory for available libraries?
const LIBRARIES: &'static [(&str, bool)] = &[ // name, enabled by default
	("FFXIVClientStructs", true),
	("Dalamud", true),
	("ImGui.NET", true),
	("ImGuiScene", true),
	("Lumina", true),
	("Lumina.Excel", true),
	("SharpDX", false),
	("SharpDX.Mathematics", false),
	("Newtonsoft.Json", false),
	("CheapLoc", false)
];

// Tab enum

#[derive(PartialEq)]
pub enum Tab {
	Basics,
	Dependencies,
	Services
}

// Gui

pub struct Gui {
	pub tab: Tab,

	pub name: String,
	pub name_internal: String,
	pub description: String,
	pub author: String,
	pub tags: String,

	pub libraries: Vec<(String, bool)>, // name, enabled

	pub service_class: String,
	pub services: Vec<Service>
}

impl Gui {
	pub fn new() -> Self {
		let mut libraries = Vec::new();

		for pair in LIBRARIES {
			libraries.push((pair.0.to_string(), pair.1));
		}

		Self {
			tab: Tab::Basics,

			name: "".to_string(),
			name_internal: "".to_string(),
			description: "".to_string(),
			author: "".to_string(),
			tags: "".to_string(),

			libraries,

			service_class: "Services".to_string(),
			services: Service::get_all()
		}
	}

	// UI helpers

	pub fn heading(ui: &mut Ui, text: &str) {
		ui.label(RichText::new(text).heading().strong());
	}

	pub fn bold(ui: &mut Ui, text: &str) {
		ui.label(RichText::new(text).strong());
	}

	// Tabs

	fn tab_basics(&mut self, ui: &mut Ui) {
		Gui::heading(ui, "Basics");

		ui.separator();

		Gui::bold(ui, "Name");
		ui.label("The plugin's title. This will be shown in the plugin installer.");
		ui.text_edit_singleline(&mut self.name);

		Gui::bold(ui, "Internal Name");
		ui.label("This will be the plugin's namespace.");
		ui.text_edit_singleline(&mut self.name_internal);

		Gui::bold(ui, "Description");
		ui.text_edit_multiline(&mut self.description);

		ui.separator();

		Gui::bold(ui, "Author");
		ui.label("That's you.");
		ui.text_edit_singleline(&mut self.author);

		Gui::bold(ui, "Tags");
		ui.text_edit_multiline(&mut self.tags);
	}

	fn tab_deps(&mut self, ui: &mut Ui) {
		Gui::heading(ui, "Dependencies");

		ui.separator();

		for pair in &mut self.libraries {
			ui.checkbox(&mut pair.1, pair.0.to_owned());
		}
	}

	fn tab_services(&mut self, ui: &mut Ui) {
		Gui::heading(ui, "Services");

		ui.separator();

		Gui::bold(ui, "Class Name");
		ui.text_edit_singleline(&mut self.service_class);

		ui.separator();

		for service in &mut self.services {
			let name = service.name.to_owned();
			ui.checkbox(&mut service.enabled, name);
		}
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
			if ui.selectable_label(self.tab == Tab::Services, "Services").clicked()
				{ self.tab = Tab::Services; }
		});

		// Draw tab

		egui::CentralPanel::default().show(ctx, |_ui| {
			egui::ScrollArea::vertical().show(_ui, |ui| {
				let spacing = ui.spacing_mut();
				spacing.item_spacing.y = 10.0;

				match self.tab {
					Tab::Basics => self.tab_basics(ui),
					Tab::Dependencies => self.tab_deps(ui),
					Tab::Services => self.tab_services(ui),
					_ => {}
				}
			});
		});
	}
}