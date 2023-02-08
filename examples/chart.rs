#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use std::f64::consts::TAU;

use eframe::egui;
use egui::*;
use plot::{BoxPlot, BoxSpread, BoxElem, Legend, Plot, Line, Points};

struct ChartsDemo {
    vertical: bool,
}

impl Default for ChartsDemo {
    fn default() -> Self {
        Self { 
            vertical: true,
        }
    }
}

impl ChartsDemo {
    fn ui(&mut self, ui: &mut Ui) -> Response {
        let yellow = Color32::from_rgb(248, 252, 168);
        // let mut box1 = BoxPlot::new(vec![
        //     BoxElem::new(0.5, BoxSpread::new(1.5, 2.2, 2.5, 2.6, 3.1)).name("Day 1"),
        //     BoxElem::new(2.5, BoxSpread::new(0.4, 1.0, 1.1, 1.4, 2.1)).name("Day 2"),
        //     BoxElem::new(4.5, BoxSpread::new(1.7, 2.0, 2.2, 2.5, 2.9)).name("Day 3"),
        // ])
        // .name("Experiment A");

        // let mut box2 = BoxPlot::new(vec![
        //     BoxElem::new(1.0, BoxSpread::new(0.2, 0.5, 1.0, 2.0, 2.7)).name("Day 1"),
        //     BoxElem::new(3.0, BoxSpread::new(1.5, 1.7, 2.1, 2.9, 3.3))
        //         .name("Day 2: interesting")
        //         .stroke(Stroke::new(1.5, yellow))
        //         .fill(yellow.linear_multiply(0.2)),
        //     BoxElem::new(5.0, BoxSpread::new(1.3, 2.0, 2.3, 2.9, 4.0)).name("Day 3"),
        // ])
        // .name("Experiment B");

        // let mut box3 = BoxPlot::new(vec![
        //     BoxElem::new(1.5, BoxSpread::new(2.1, 2.2, 2.6, 2.8, 3.0)).name("Day 1"),
        //     BoxElem::new(3.5, BoxSpread::new(1.3, 1.5, 1.9, 2.2, 2.4)).name("Day 2"),
        //     BoxElem::new(5.5, BoxSpread::new(0.2, 0.4, 1.0, 1.3, 1.5)).name("Day 3"),
        // ])
        // .name("Experiment C");

        // if !self.vertical {
        //     box1 = box1.horizontal();
        //     box2 = box2.horizontal();
        //     box3 = box3.horizontal();
        // }
        let n = 50;
        let mut sin_values: Vec<_> = (0..=n)
            .map(|i| remap(i as f64, 0.0..=n as f64, -TAU..=TAU))
            .map(|i| [i, i.sin()])
            .collect();
        let mut sin_values2: Vec<_> = sin_values.clone().iter().map(|xy| [xy[0], xy[1] + 2.0]).collect();
        let line = Line::new(sin_values.clone()).fill(-1.5);
        let points = Points::new(sin_values).stems(-1.0).radius(1.5);
        let points2 = Points::new(sin_values2.split_off(n/2)).radius(2.0).color(yellow);

        Plot::new("Box Plot Demo")
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                // plot_ui.box_plot(box1);
                // plot_ui.box_plot(box2);
                // plot_ui.box_plot(box3);
                plot_ui.points(points.name("Points with stems"));
                plot_ui.points(points2.name("Points2"));
                plot_ui.line(line.name("Line with fill"));
            })
            .response
    }
}

#[derive(Default)]
struct MyApp {
    box_chart: ChartsDemo,
    show: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { show, box_chart } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("The box chart");
            ui.horizontal(|ui| {
                ui.checkbox(show, "show the chart");
                // ui.checkbox(&mut box_chart.vertical, "vertical")
                //     .on_hover_text("Click to change box");
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
            });
            
            if *show {
                box_chart.ui(ui);
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Plots Demo",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}