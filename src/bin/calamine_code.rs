// use std::error::Error;

// use calamine::{open_workbook, DataType, Reader, Xlsx};

// use calamine::XlsxError;

// fn main() {
//     if let Err(e) = read_excel() {
//         eprintln!("there is some problem {}", e);
//         std::process::exit(1);
//     }
// }

// fn read_excel() -> Result<(), XlsxError> {
//     let mut workbook: Xlsx<_> = open_workbook("TaxBreakets.xlsx")?;
//     if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
//         // println!("range : {:?}", range);
//         let start = range.start().unwrap();
//         let end = range.end().unwrap();

//         println!("range start : {:?}\t end: {:?}", start, end);

//         let a = range.get_value((0, 0)).unwrap();

//         // println!("A(1,1) : {:?}", a);

//         match a {
//             DataType::Int(_) => todo!(),
//             DataType::Float(_) => todo!(),
//             DataType::String(title) => println!("title : {:?},", title),
//             DataType::Bool(_) => todo!(),
//             DataType::DateTime(_) => todo!(),
//             DataType::Duration(_) => todo!(),
//             DataType::DateTimeIso(_) => todo!(),
//             DataType::DurationIso(_) => todo!(),
//             DataType::Error(_) => todo!(),
//             DataType::Empty => todo!(),
//         }
//     }

//     return Ok(());
// }
