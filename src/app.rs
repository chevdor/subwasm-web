use eframe::{
	egui::{CentralPanel, Context},
	App, Frame,
};
use egui_file::FileDialog;
use log::info;
use std::path::PathBuf;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SubwasmApp {
	// // Example stuff:
	// label: String,

	// // this how you opt-out of serialization of a member
	// #[serde(skip)]
	// value: f32,
	file_1: Option<PathBuf>,

	#[serde(skip)]
	open_file_dialog: Option<FileDialog>,
}

impl Default for SubwasmApp {
	fn default() -> Self {
		Self {
			// Example stuff:
			// label: "Hello World!".to_owned(),
			// value: 2.7,
			file_1: None,
			open_file_dialog: None,
		}
	}
}

impl SubwasmApp {
	/// Called once before the first frame.
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// This is also where you can customize the look and feel of egui using
		// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

		// Load previous app state (if any).
		// Note that you must enable the `persistence` feature for this to work.
		if let Some(storage) = cc.storage {
			return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
		}

		Default::default()
	}
}

impl eframe::App for SubwasmApp {
	/// Called by the frame work to save state before shutdown.
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}

	/// Called each time the UI needs repainting, which may be many times per second.
	/// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let Self {
			// label, value
			file_1,
			open_file_dialog,
		} = self;

		// Examples of how to create different panels and windows.
		// Pick whichever suits you.
		// Tip: a good default choice is to just keep the `CentralPanel`.
		// For inspiration and more examples, go to https://emilk.github.io/egui

		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			// The top panel is often a good place for a menu bar:
			egui::menu::bar(ui, |ui| {
				ui.menu_button("File", |ui| {
					#[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
					if ui.button("Quit").clicked() {
						_frame.close();
					}

					if (ui.button("Open")).clicked() {
						let mut dialog = FileDialog::open_file(self.file_1.clone());
						dialog.open();
						self.open_file_dialog = Some(dialog);
					}

					// if let Some(dialog) = &mut self.open_file_dialog {
					//   if dialog.show(ctx).selected() {
					//     if let Some(file) = dialog.path() {
					//       self.file_1 = Some(file);
					//     }
					//   }
					// }

					if ui.button("About...").clicked() {
						info!("About...");
					}
				});
			});
		});

		// CentralPanel::default().show(ctx, |ui| {});

		// egui::SidePanel::left("side_panel").show(ctx, |ui| {
		//     ui.heading("Side Panel");

		//     ui.horizontal(|ui| {
		//         ui.label("Write something: ");
		//         ui.text_edit_singleline(label);
		//     });

		//     ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
		//     if ui.button("Increment").clicked() {
		//         *value += 1.0;
		//     }

		//     ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
		//         ui.horizontal(|ui| {
		//             ui.spacing_mut().item_spacing.x = 0.0;
		//             // TODO: bring back credits for the great work in a proper place
		//             // ui.label("powered by ");
		//             // ui.hyperlink_to("egui", "https://github.com/emilk/egui");
		//             // ui.label(" and ");
		//             // ui.hyperlink_to(
		//             //     "eframe",
		//             //     "https://github.com/emilk/egui/tree/master/crates/eframe",
		//             // );
		//             // ui.label(".");
		//         });
		//     });
		// });

		egui::CentralPanel::default().show(ctx, |ui| {
			// The central panel the region left after adding TopPanel's and SidePanel's

			ui.heading("Subwasm Web");
			ui.hyperlink("https://github.com/chevdor/subwasm");
			// ui.add(egui::github_link_file!(
			//     "https://github.com/chevdor/subwasm",
			//     "Source code."
			// ));
			egui::warn_if_debug_build(ui);
		});

		if false {
			egui::Window::new("Window").show(ctx, |ui| {
				ui.label("Windows can be moved by dragging them.");
				ui.label("They are automatically sized based on contents.");
				ui.label("You can turn on resizing and scrolling if you like.");
				ui.label("You would normally choose either panels OR windows.");
			});
		}
	}
}
