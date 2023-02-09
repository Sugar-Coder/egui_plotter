use std::f64::consts::TAU;

use calamine::{Range, DataType};
use eframe::egui;
use egui::*;
use plot::{Legend, Plot, Line, Points};

pub struct ChartsDemo {
    rows: Option<Range<DataType>>,
    pub filename: Option<String>,
}

impl Default for ChartsDemo {
    fn default() -> Self {
        Self { 
            rows: None,
            filename: None,
        }
    }
}

impl ChartsDemo {
    // todo add update data method
    pub fn ui(&mut self, ui: &mut Ui) -> Response {
        let yellow = Color32::from_rgb(248, 252, 168);
        let n = 50;
        let sin_values: Vec<_> = (0..=n)
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
                plot_ui.points(points.name("Points with stems"));
                plot_ui.points(points2.name("Points2"));
                plot_ui.line(line.name("Line with fill"));
            })
            .response
    }

    pub fn load_excel_data(&mut self, filename: String, rows: Option<Range<DataType>>) {
        self.rows = rows;
        self.filename = Some(filename);
    }

    pub fn clear(&mut self) {
        self.rows = None;
        self.filename = None;
    }
}
