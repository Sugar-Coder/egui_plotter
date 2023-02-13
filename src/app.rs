use crate::{ChartsDemo, reader::{read_excel, read_excel_wasm}};
use rfd::AsyncFileDialog;  // for wasm


pub struct TemplateApp {
    info_label: String,
    chart_demo: ChartsDemo,
    data_channel: (
        std::sync::mpsc::Sender<Vec<u8>>,
        std::sync::mpsc::Receiver<Vec<u8>>,
    ),  // for wasm rfd open file async
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            info_label: "Select file and plot".to_string(),
            chart_demo: ChartsDemo::default(),
            data_channel: std::sync::mpsc::channel(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.label(format!("{}", self.info_label));
            ui.horizontal(|ui| {
                if ui.button("Open File").clicked() {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(path) = rfd::FileDialog::new().add_filter("excel", &["xlsx"]).pick_file() {
                        let filepath = path.display().to_string();
                        if let Some(excel_data) = read_excel(&filepath) {
                            // successfully read file
                            self.chart_demo.load_excel_data(path.display().to_string(), excel_data);
                            self.info_label = format!("{} opened", filepath);
                        } else {
                            self.info_label = format!("cannot open {}", filepath);
                        }
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        let task = AsyncFileDialog::new()
                                .add_filter("excel", &["xlsx"])
                                .set_directory("/")
                                .pick_file();
                        let data_sender = self.data_channel.0.clone();
                        let ctx = ctx.clone(); // clone the ctx for closure
                        execute(async move {
                            let file = task.await;
                            if let Some(file) = file {
                                // If you are on native platform you can just get the path
                                // #[cfg(not(target_arch = "wasm32"))]
                                // println!("{:?}", file.path());
                    
                                // If you care about wasm support you just read() the file
                                let raw_data = file.read().await;
                                data_sender.send(raw_data).ok();
                                ctx.request_repaint(); // wake up UI thread
                            }
                        });
                    }
                }
                if ui.button("Close file").clicked() {
                    self.chart_demo.clear();
                    self.info_label = "Select file and plot".to_string();
                }
            });
            ui.collapsing("Instructions", |ui| {
                ui.label("Pan by dragging, or scroll (+ shift = horizontal).");
                ui.label("Box zooming: Right click to zoom in and zoom out using a selection.");
                if cfg!(target_arch = "wasm32") {
                    ui.label("Zoom with ctrl / ⌘ + pointer wheel, or with pinch gesture.");
                } else if cfg!(target_os = "macos") {
                    ui.label("Zoom with ctrl / ⌘ + scroll.");
                } else {
                    ui.label("Zoom with ctrl + scroll.");
                }
                ui.label("Reset view with double-click.");
            });
            egui::warn_if_debug_build(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui|{
            #[cfg(target_arch = "wasm32")]
            match self.data_channel.1.try_recv() {
                Ok(rdata) => {
                    // Process FileOpen and other messages
                    if let Some(excel_data) = read_excel_wasm(rdata) {
                        // successfully read file
                        self.chart_demo.load_excel_data("excel_file".to_string(), excel_data);
                        self.info_label = "excel file opened".to_string();
                    } else {
                        self.info_label = "cannot open excel file".to_string();
                    }
                    println!("Got excel data");
                }
                Err(_) => (),
            }
            if self.chart_demo.filename != None {
                self.chart_demo.ui(ui);
            }
        });
    }
}

use std::future::Future;

// for test locally for the AsyncFileDialog
#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    // this is stupid... use any executor of your choice instead
    std::thread::spawn(move || futures::executor::block_on(f));
}

#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}