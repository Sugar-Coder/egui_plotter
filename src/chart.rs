use calamine::{Range, DataType};
use eframe::egui;
use egui::*;
use plot::{Legend, Plot, Points};

struct NamedColumn {
    name: String,
    data: Vec<DataType>,
}

impl NamedColumn {
    fn new(name: String, data: Vec<DataType>) -> Self {
        Self { name: name, data: data }
    }
}

pub struct ChartsDemo {
    // r: Option<Range<DataType>>,
    pub filename: Option<String>,
    columns: Vec<NamedColumn>,
}

impl Default for ChartsDemo {
    fn default() -> Self {
        Self { 
            filename: None,
            columns: Vec::new(),
        }
    }
}

impl ChartsDemo {
    // todo add update data method
    pub fn ui(&mut self, ui: &mut Ui) -> Response {
        let row_num = self.columns[0].data.len();
        let mut column_name_vec: Vec<&String> = Vec::new();
        let mut points_vec: Vec<Points> = Vec::new();
        for col in &self.columns {
            if col.name != "date" && col.data[0].is_float() {
                let values: Vec<_> = (0..row_num)
                    .map(|i| [i as f64, col.data[i].get_float().unwrap()])
                    .collect();
                // vals_vec.push(values);
                points_vec.push(Points::new(values).radius(1.5));
                column_name_vec.push(&col.name);
            }
        }

        Plot::new("Box Plot Demo")
            .legend(Legend::default())
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
                self.columns.push(NamedColumn::new(col_name.to_string(), col_data));
            }
        }
    }

    pub fn clear(&mut self) {
        self.filename = None;
        self.columns.clear();
    }
}

#[test]
fn test_parse_column() {
    let mut demo = ChartsDemo::default();
    use crate::reader::read_excel;
    let path = "/Users/sjy/Downloads/former_reinstatement.xlsx".to_string();
    demo.load_excel_data(path.clone(), read_excel(&path).unwrap());

    for col in &demo.columns {
        println!("col:{}, size={}", col.name, col.data.len());
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