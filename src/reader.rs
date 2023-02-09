use calamine::{open_workbook, Xlsx, Reader, Range, DataType};

pub fn read_excel(path: &String) -> Option<Range<DataType>> {
    // let path = "/Users/sjy/Downloads/former_reinstatement.xlsx".to_string();
    let mut excel: Xlsx<_> = match open_workbook(path) {
        Ok(e) => e,
        Err(_) => return None,
    };
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        Some(r)
    } else {
        None
    }
}