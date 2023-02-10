use calamine::{open_workbook, Xlsx, Reader};

fn main() {
    let path = "/Users/sjy/Downloads/former_reinstatement.xlsx".to_string();
    let mut excel: Xlsx<_> = open_workbook(path).unwrap();
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        let mut count = 5;
        let first_row = r.rows().next().unwrap();
        println!("{}, {}", first_row[0].is_string(), first_row[0].to_string() == "date");
        // for row in r.rows() {
        //     println!("row={:?}, row[0]={:?}", row, row[0]);
        //     count -= 1;
        //     if count <= 0 {
        //         break;
        //     }
        // }
    }
}