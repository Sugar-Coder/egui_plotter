use std::collections::HashMap;
use std::ops::RangeInclusive;
use calamine::{Range, DataType};
use eframe::egui;
use egui::*;
use plot::{Legend, Plot, Points, PlotPoint};
use chrono::{NaiveDate, Datelike};


pub struct ChartsDemo {
    pub filename: Option<String>, // the excel file name
    columns: HashMap<String, Vec<DataType>>,
}

impl Default for ChartsDemo {
    fn default() -> Self {
        Self { 
            filename: None,
            columns: HashMap::new(),
        }
    }
}

impl ChartsDemo {
    // todo add update data method
    pub fn ui(&mut self, ui: &mut Ui) -> Response {
        let row_num = self.columns["date"].len();
        let mut column_name_vec: Vec<&String> = Vec::new();
        let mut points_vec: Vec<Points> = Vec::new();
        
        let begin_date = self.columns["date"][0].to_string();
        let begin_nums_day = date_to_num_days(&begin_date);
        let x_axis_value: Vec<i32> = self.columns["date"].iter()
                                            .map(|s| date_to_num_days(&s.to_string()) - begin_nums_day)
                                            .collect();
        

        for (col_name, col_data) in &self.columns {
            if col_name != "date" && col_data[0].is_float() {
                let values: Vec<_> = (0..row_num)
                    .map(|i| [x_axis_value[i] as f64, col_data[i].get_float().unwrap()])
                    .collect();
                // vals_vec.push(values);
                points_vec.push(Points::new(values).radius(1.5));
                column_name_vec.push(&col_name);
            }
        }
        // 使用move，让closure能使用外部参数
        let x_fmt = move |x: f64, range: &RangeInclusive<f64>| {
            if x < 0.0 || x >= *range.end() {
                // No labels outside value bounds
                String::new()
            } else {
                // Date
                let date = NaiveDate::from_num_days_from_ce_opt(begin_nums_day + x as i32).unwrap();
                format!("{}", date.to_string())
            }
        };

        let label_fmt = move |name: &str, val: &PlotPoint| {
            format!(
                "{}:{}\n{}",
                name,
                NaiveDate::from_num_days_from_ce_opt(begin_nums_day + val.x as i32).unwrap().to_string(),
                val.y,
            )
        };

        Plot::new("Box Plot Demo")
            .legend(Legend::default())
            .x_axis_formatter(x_fmt)
            .label_formatter(label_fmt)
            .show(ui, |plot_ui| {
                loop {
                    if let Some(points) = points_vec.pop() {
                        let name = column_name_vec.pop().unwrap();
                        plot_ui.points(points.name(name));
                    } else {
                        break;
                    }
                }
            })
            .response
    }

    fn parse_column(&mut self, col_name: String, r: &Range<DataType>) -> Option<Vec<DataType>> {
        let mut rows = r.rows();
        if let Some(first_row) = rows.next() {
            let mut index: usize = first_row.len();
            for (i, cell) in first_row.iter().enumerate() {
                if cell.is_string() && cell.to_string() == col_name {
                    index = i;
                    break;
                }
            }
            // cannot find the column
            if index == first_row.len() {
                return None;
            }
            let col_data: Vec<_> = rows.map(|row| row[index].clone()).collect();
            return Some(col_data);
        }
        None
    }

    pub fn load_excel_data(&mut self, filename: String, r: Range<DataType>) {
        self.filename = Some(filename);
        let column_names = vec!["date", "high", "low", "open", "close"];
        for col_name in column_names {
            if let Some(col_data) = self.parse_column(col_name.to_string(), &r) {
                // self.columns.push(NamedColumn::new(col_name.to_string(), col_data));
                self.columns.insert(col_name.to_string(), col_data);
            }
        }
    }

    pub fn clear(&mut self) {
        self.filename = None;
        self.columns.clear();
    }
}


/// parse "2021-12-31" like date to days from ce
fn date_to_num_days(date: &str) -> i32 {
    let dt = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    return dt.num_days_from_ce();
}

#[test]
fn test_date2days() {
    println!("{}", date_to_num_days(&"2021-12-21".to_string()));
}

#[test]
fn test_parse_column() {
    let mut demo = ChartsDemo::default();
    use crate::reader::read_excel;
    let path = "/Users/sjy/Downloads/former_reinstatement.xlsx".to_string();
    demo.load_excel_data(path.clone(), read_excel(&path).unwrap());

    for (col_name, col_data) in &demo.columns {
        println!("col:{}, size={}", col_name, col_data.len());
    }
    // let col = &demo.columns[0];
    // println!("column {}", col.name);
    // let mut count = 5;
    // for cell in &col.data {
    //     if count > 0 {
    //         println!("{}", cell);
    //         count -= 1;
    //     }
    // }
}