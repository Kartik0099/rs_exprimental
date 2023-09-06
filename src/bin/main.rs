use std::error::Error;

use umya_spreadsheet;

use umya_spreadsheet::reader::xlsx::XlsxError;

// struct

fn main() {
    if let Err(e) = read_excel() {
        eprintln!("there is some problem {}", e);
        std::process::exit(1);
    }
}

fn read_excel() -> Result<(), XlsxError> {
    let range = "C5:F9";
    let row = 1;
    let column = 3;

    let path = std::path::Path::new("TaxBreakets.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::lazy_read(path)?;
    let mut res_worksheet = book.get_sheet_by_name_mut("Sheet1");

    if let Ok(worksheet) = res_worksheet {
        let value: std::borrow::Cow<'_, str> = worksheet.get_cell_value((row, column)).get_value();

        match value {
            std::borrow::Cow::Borrowed(v) => println!("value is {}", v),
            std::borrow::Cow::Owned(o) => println!("value is owned {}", o),
        }

        println!("value is  empty");
    } else if let Err(e) = res_worksheet {
        println!("Error {}", e);
    }

    // let value = cells.get_cell_value((&5, &12)).get_value();
    // let value = cells.get_cell_value("E12").get_value();
    return Ok(());
}
