use eframe::egui;
use egui::{CentralPanel, Ui};
use log::info;
use poll_promise::Promise;
use std::path::PathBuf;

use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
	// this is stupid... use any executor of your choice instead
	std::thread::spawn(move || futures::executor::block_on(f));
}
// #[cfg(target_arch = "wasm32")]
// fn execute<F: Future<Output = ()> + 'static>(f: F) {
//     wasm_bindgen_futures::spawn_local(f);
// }

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
// if we add new fields, give them default values when deserializing old state
#[serde(default)]
pub struct SubwasmApp {
	data_1: Option<Vec<u8>>,
	// #[serde(skip)]
	// open_file_dialog: Option<FileDialog>,
	#[serde(skip)]
	promise: Option<Promise<Vec<u8>>>,
	#[serde(skip)]
	show_about: bool,

	#[serde(skip)]
	pub windows: Vec<SubwasmApp>,
}

impl Default for SubwasmApp {
	fn default() -> Self {
		Self {
			// Example stuff:
			// label: "Hello World!".to_owned(),
			// value: 2.7,
			data_1: None,
			// open_file_dialog: None,
			promise: None,

			show_about: false,

			windows: vec![],
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

impl SubwasmApp {
	pub fn name(&self) -> &'static str {
		"Subwasm Web"
	}

	pub fn show_about(&mut self, ui: &mut Ui, open: &mut bool) {
		egui::Window::new("About...").show(ui.ctx(), |ui| {
			ui.label("Windows can be moved by dragging them.");
			ui.label("They are automatically sized based on contents.");
			ui.label("You can turn on resizing and scrolling if you like.");
			ui.label("You would normally choose either panels OR windows.");
		});

		// egui::Window::new(self.name()).open(open).show(ui.ctx(), |ui|
		// 	self.ui(ui));
	}

	pub fn ui(&mut self, ui: &mut Ui) {
		ui.label("Contract ID");
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
			data_1,
			// open_file_dialog,
			promise: None,
			show_about: false,
			windows,
		} = self
		 else {
			todo!()
		};

		// let promise = self.promise.get_or_insert_with(|| {
		//     // Begin download.
		//     // We download the image using `ehttp`, a library that works both in WASM and on native.
		//     // We use the `poll-promise` library to communicate with the UI thread.
		//     let ctx = ctx.clone();
		//     let (sender, promise) = Promise::new();
		//     // let request = ehttp::Request::get("https://picsum.photos/seed/1.759706314/1024");
		//     // ehttp::fetch(request, move |response| {
		//     //     let image = response.and_then(parse_response);
		//     //     sender.send(image); // send the results back to the UI thread.
		//     //     ctx.request_repaint(); // wake up UI thread
		//     // });
		//     promise
		// });

		// ctx.request_repaint();

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
						log::debug!("Open...");

						let task = rfd::AsyncFileDialog::new().pick_file();

						// execute(async {
						// 	let file = task.await;

						// 	if let Some(file) = file {
						// 		// If you are on native platform you can just get the path
						// 		#[cfg(not(target_arch = "wasm32"))]
						// 		println!("{:?}", file.path());

						// 		// If you care about wasm support you just read() the file
						// 		let res =	 file.read().await;

						// 		// self.data_1 = Some(res);
						// 		info!("Got file: {:?}", res.len());

						// 		// todo: https://github.com/emilk/egui/blob/master/examples/download_image/src/main.rs

						// 	} else {
						// 		info!("Got no file");
						// 	}
						// });
						// let future = async {
						// 	let file = rfd::AsyncFileDialog::new()
						// 		.add_filter("text", &["txt", "rs"])
						// 		.add_filter("rust", &["rs", "toml"])
						// 		.set_directory("/")
						// 		.pick_file()
						// 		.await;

						// 	let data = file.unwrap().read().await;
						// };

						// file_1 = rfd::FileDialog::new()
						// 	.add_filter("text", &["txt", "rs"])
						// 	.add_filter("rust", &["rs", "toml"])
						// 	.set_directory("/")
						// 	.pick_file();
						// let mut dialog = FileDialog::open_file(self.file_1.clone());
						// dialog.open();
						// self.open_file_dialog = Some(dialog);
					}

					// if let Some(dialog) = &mut self.open_file_dialog {
					//   if dialog.show(ctx).selected() {
					// 	if let Some(file) = dialog.path() {
					// 	  self.file_1 = Some(file);
					// 	}
					//   }
					// }

					if ui.button("About...").clicked() {
						println!("About"); // native
						info!("About..."); // web
				   // SubwasmApp::default().show_about(ui, &mut true)
						self.windows.push(SubwasmApp::default());
					}
					for contract in self.windows.iter_mut() {
						let mut is_open = true; // usually, you will store this within the Contractor struct or such. otherwise, there's no point in even passing this
						contract.show_about(ui, &mut is_open); // show the window. remember that you will want a different name for each window, as egui will use the name as the id source (you can also pass in a id manually if you want, but avoided it here to keep this small).
						               // is_open will be false if user clicks the red close button, but i don't want the example to be too big
					}
				});
			});
		});

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

		// egui::TopBottomPanel::bottom("bot").show(ctx, |ui| match promise.ready() {
		// 	// egui::Window::new("Log").show(ctx, |ui| {
		// 	// 	egui_logger::logger_ui(ui);
		// 	// 	// if ui.button("Logs").clicked() {
		// 	// 	// }
		// 	// });
		// 	None => {
		//         ui.spinner(); // still loading
		//     }
		//     Some(_x) => {
		//         ui.colored_label(ui.visuals().error_fg_color, _x.len().to_string()); // something went wrong
		//     }

		// });

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
