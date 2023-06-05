#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    use log::debug;

    env_logger::init();
    debug!("this is a debug {}", "message");

    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();
    println!("test");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Subwasm Web",
        native_options,
        Box::new(|cc| Box::new(subwasm_web::SubwasmApp::new(cc))),
    )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    use log::debug;

    env_logger::init();
    debug!("this is a debug {}", "message");
    println!("test");

    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(subwasm_web::SubwasmApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
