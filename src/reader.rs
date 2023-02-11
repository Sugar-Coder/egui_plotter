use calamine::{open_workbook, open_workbook_auto_from_rs, Xlsx, Reader, Range, DataType};
use std::io::Cursor;

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

pub fn read_excel_wasm(data: Vec<u8>) -> Option<Range<DataType>> {
    let mut workbook = match open_workbook_auto_from_rs(Cursor::new(data)) {
        Ok(w) => w,
        Err(_) => return None,
    };
    if let Some(Ok(r)) = workbook.worksheet_range("Sheet1") {
        Some(r)
    } else {
        None
    }
}